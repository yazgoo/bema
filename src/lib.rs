use tempfile::Builder;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::{SystemTime, Duration};
use std::process::Command;
use std::fs::File;
use std::env;
use blockish::render_image;
use image::io::Reader as ImageReader;

use crossterm::{
    execute,
    cursor::{MoveTo, MoveRight, Hide, Show},
    event::{Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Attribute, SetAttribute},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType, enable_raw_mode, disable_raw_mode, self},
    ExecutableCommand, Result,
    event,
};

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use macroquad::prelude::*;

#[derive(Clone)]
enum SlideItem {
    Code{ extension: String, source: String },
    Image{ image: &'static [u8], extension: String, width: Option<usize> },
    Text{ text: String },
}

#[derive(Clone)]
pub struct Slide {
    title: String,
    items: Vec<SlideItem>,
}

#[derive(Clone)]
pub struct Bema {
    slides: Vec<Slide>
}

trait Runner {
    fn run(&self, bema: &Bema) -> Result<()>;
}

struct TerminalRunner {
}


fn display_image(image_path: &String) {
    match env::var("KITTY_WINDOW_ID") {
        Ok(_) => {
            let _res = Command::new("kitty")
                .arg("+kitten")
                .arg("icat")
                .arg(image_path)
                .output();
        },
        Err(_) => {
            render_image(image_path, 100 * 4);
        }
    }
}

fn get_justify(size: usize, texts: Vec<&String>) -> Result<usize> {

    let mut whitespaces : usize = size as usize;

    for text in texts {
        let new_whitespaces = if text.len() < size as usize {
            let x = (size as usize - text.len()) / 2;
                x
        } else {
            0
        };
        if new_whitespaces == 0 {
            whitespaces = 0;
            break
        } else if new_whitespaces < whitespaces {
                whitespaces = new_whitespaces
        }
    }

    Ok(whitespaces)


}

fn justify_center(size: usize, text: Vec<&String>) -> Result<()> {
    let whitespaces = get_justify(size, text)?;
    stdout()
        .execute(MoveRight(whitespaces as u16))?;
    Ok(())
}

impl TerminalRunner {
    fn clear_screen(&self) -> Result<()> {

        stdout()
            .execute(Clear(ClearType::All))?;

        Ok(())
    }

    fn read_char(&self) -> Result<char> {
        enable_raw_mode()?;
        loop {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) = event::read()?
            {
                disable_raw_mode()?;
                return Ok(c);
            }
        }
    }

    fn render_slide(&self, slide: &Slide) -> Result<()> {

        justify_center(terminal::size()?.0 as usize, vec![&slide.title])?;

        stdout()
        .execute(SetAttribute(Attribute::Bold))?
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Black))?
        .execute(Print(slide.title.to_string()))?
        .execute(ResetColor)?
        .execute(Print("\n\n"))?;

        for item in &slide.items {
            match item {
                SlideItem::Image { image, extension, width: _ } => {
                    let mut file = Builder::new()
                        .prefix("image")
                        .suffix(extension)
                        .rand_bytes(5)
                        .tempfile()?;
                    file.write(image)?;
                    display_image(&file.path().to_str().unwrap().to_string());
                },
                SlideItem::Code { extension, source } => {
                    let ps = SyntaxSet::load_defaults_newlines();
                    let ts = ThemeSet::load_defaults();

                    let syntax = ps.find_syntax_by_extension(extension).unwrap();
                    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
                    let splits = source.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
                    let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
                    let whitespaces = get_justify(terminal::size()?.0 as usize, v2)? as usize;
                    for line in LinesWithEndings::from(source) {
                        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
                        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                        stdout()
                            .execute(ResetColor)?
                            .execute(MoveRight(whitespaces as u16))?;
                        print!("{}", escaped);
                    }

                    stdout()
                        .execute(ResetColor)?;
                    },
                SlideItem::Text { text } => {
                    let splits = text.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
                    let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
                    let whitespaces = get_justify(terminal::size()?.0 as usize, v2)?;
                    for split in splits {
                        stdout().execute(MoveRight(whitespaces as u16))?;
                        println!("{}", split);
                    }
                },
            }
            //let _ = item.render();
        }

        Ok(())
    }

}

impl Runner for TerminalRunner {

    fn run(&self, bema: &Bema) -> Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        self.clear_screen()?;

        execute!(
            stdout(),
            Hide
        )?;

        let mut i : i16 = -1;
        loop {
            if i >= 0 {
                let c = self.read_char()?;
                match c {
                    'g' => i = 0,
                    'G' => i = bema.slides.len() as i16 - 1,
                    'n'|'j'|'l' => i+=1,
                    'p'|'k'|'h' => i-=1,
                    'q' => break,
                    _ => {}
                }
                if i as usize >= bema.slides.len() {
                    i = 0;
                }
                if i < 0 {
                    i = bema.slides.len() as i16 - 1;
                }
            } else {
                i = 0;
            }
            execute!(
                stdout(),
                MoveTo(0, 0),
            )?;
            self.clear_screen()?;
            println!("{}/{}", i + 1, bema.slides.len());
            self.render_slide(bema.slides.get(i as usize).unwrap())?;
        }

        execute!(
            stdout(),
            Show
        )?;

        execute!(stdout(), LeaveAlternateScreen)?;

        Ok(())
    }
}

struct GuiRunner {
}

async  fn main_gui_runner(bema: Bema) {
    let font = load_ttf_font_from_bytes(include_bytes!("3270 Narrow Nerd Font Complete.ttf"));
    let mut i : i32 = 0;
    let mut slide = bema.slides.get(i as usize).unwrap();
    let mut antibounce = SystemTime::now(); 
    let mut textures = HashMap::new();

    loop {
        clear_background(BLACK);
        
        let mut changed = false;


        if antibounce.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() >= 200 {
            if is_key_down(miniquad::KeyCode::Right) {
                i += 1;
                changed = true;
            }
            if is_key_down(miniquad::KeyCode::Left) {
                i -= 1;
                changed = true;
            }
            if is_key_down(miniquad::KeyCode::Down) {
                i += 1;
                changed = true;
            }
            if is_key_down(miniquad::KeyCode::Up) {
                i -= 1;
                changed = true;
            }
            if i >= bema.slides.len() as i32 {
                i = 0;
            }
            else if i < 0 {
                i = bema.slides.len() as i32 - 1;
            }
            if changed {
                slide = bema.slides.get(i as usize).unwrap();
            }
            antibounce = SystemTime::now();
        }
        let mut y = 20.0;
        draw_text_ex(format!("{}/{}", i + 1, bema.slides.len()).as_str(), 20.0, y, TextParams { font_size: 20, font,
                                            ..Default::default()
                                                            });
        y += 80.0;

        let x = 30 * get_justify((screen_width() / 30.0) as usize, vec![&slide.title]).unwrap_or(0);
        draw_text_ex(&slide.title, x as f32, y, TextParams { font_size: 80, font,
                                            ..Default::default()
                                                            });
        y += 180.0;
            for (pos, item) in slide.items.iter().enumerate() {
                match item {
                    SlideItem::Image { image: bytes, extension, width: _ } => {
                        match textures.get(&(i, pos)) {
                            Some(_) => {},
                            None => {
                                let quad_context = unsafe { get_internal_gl() }.quad_context;
                                let texture = if extension == ".jpg" {
                                    let img = ImageReader::with_format(std::io::Cursor::new(bytes), image::ImageFormat::Jpeg).decode().unwrap();
                                    let mut bytes: Vec<u8> = Vec::new();
                                    img.write_to(&mut bytes, image::ImageOutputFormat::Png).unwrap();
                                    Texture2D::from_file_with_format(quad_context, &bytes[..], None)
                                } else {
                                    Texture2D::from_file_with_format(quad_context, &bytes[..], None)
                                };
                                textures.insert((i, pos), texture);
                            }
                        };
                        let texture = *textures.get(&(i, pos)).unwrap();
                        let w = screen_width();
                        let x = if w < texture.width() {
                            0.0
                        } else {
                            (w - texture.width()) / 2.0
                        };
                        draw_texture(texture, x, y, WHITE);
                    },
                    SlideItem::Code { extension: _, source } => {
                        let splits = source.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
                        let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
                        let x = 40 * get_justify((screen_width() / 30.0) as usize, v2).unwrap();
                        for split in splits {
                            draw_text_ex(&split, x as f32, y, TextParams { font_size: 40, font,
                                ..Default::default()
                            });
                            y += 35.0;
                        }
                    },
                    SlideItem::Text { text } => {
                        let splits = text.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
                        let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
                        let x = 40 * get_justify((screen_width() / 30.0) as usize, v2).unwrap();
                        for split in splits {
                            draw_text_ex(&split, x as f32, y, TextParams { font_size: 40, font,
                                ..Default::default()
                            });
                            y += 35.0;
                        }
                    },
                }
            };
        next_frame().await;
    }
}

impl Runner for GuiRunner {
    fn run(&self, bema: &Bema) -> Result<()> {

        macroquad::Window::new("Bema", main_gui_runner(bema.clone()));
        Ok(())
    }
}


struct HovercraftRunner {
}

impl Runner for HovercraftRunner {
    fn run(&self, bema: &Bema) -> Result<()> {
        for (i, slide) in bema.slides.iter().enumerate() {
            if i > 0 { println!("----"); }
            println!("");
            println!("{}", slide.title);
            for _ in 0..slide.title.len() {
                print!("=");
            }
            println!("");
            println!("");
            let mut img_i = 0;
            for item in &slide.items {
                match item {
                    SlideItem::Image { image, extension, width } => {
                        let file_path = format!("bema_{}{}", img_i, extension);
                        img_i += 1;
                        let mut buffer = File::create(&file_path)?;
                        buffer.write_all(image)?;
                        println!(".. image:: {}", &file_path);
                        width.map( |w| println!("   :width: {} px", w));
                    },
                    SlideItem::Code { extension, source } => {
                        println!(".. code:: {}", extension);
                        println!();
                        let splits = source.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
                        for split in splits {
                            println!("  {}", split);
                        }
                    },
                    SlideItem::Text { text } => {
                        println!("{}", text);
                    },
                }
            };
            println!("");
        }
        Ok(())
    }
}

pub fn slides(f: fn(Bema) -> Bema) -> Bema {
    f(Bema { 
        slides: vec![],
    })
}

impl Slide {
}


impl Bema {
    pub fn slide(mut self, title: &str, f: fn(Slide) -> Slide) -> Bema {

        let s = Slide {
            title: String::from(title),
            items: vec![],
        };
        self.slides.push(f(s));
        self
    }

    pub fn run(&self) -> Result<()> {
        if env::args().len() == 2 {
            let args : Vec<String> = env::args().collect();
            match args[1].as_str() {
                "hovercraft" => HovercraftRunner { }.run(&self)?,
                "gui" => GuiRunner { }.run(&self)?,
                _ => {}
            }
        } else {
            TerminalRunner { }.run(&self)?;
        }
        Ok(())
    }
}

impl Slide {
    pub fn text(mut self, s: &str) -> Slide {
        self.items.push(SlideItem::Text { text: String::from(s) });
        self
    }

    pub fn t(self, s: &str) -> Slide {
        self.text(s)
    }

    pub fn code(mut self, extension: &str, source: &str) -> Slide {
        self.items.push(SlideItem::Code { extension: String::from(extension), source: String::from(source) });
        self
    }

    pub fn image(mut self, image: &'static [u8], extension: &str, width: Option<usize>) -> Slide {
        self.items.push(SlideItem::Image { image, extension: String::from(extension), width });
        self
    }
}
