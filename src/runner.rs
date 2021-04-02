use crate::bema::{Bema, SlideItem, Slide};

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

use tempfile::Builder;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::time::{SystemTime, Duration};
use std::process::Command;
use std::fs::File;
use std::env;
use blockish::render_image;

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use macroquad::prelude::*;
pub trait Runner {
    fn run(&self, bema: &Bema) -> Result<()>;
}

pub struct TerminalRunner {
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

pub struct GuiRunner {
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
                    SlideItem::Code { extension, source } => {
                        let ps = SyntaxSet::load_defaults_newlines();
                        let ts = ThemeSet::load_defaults();

                        let syntax = ps.find_syntax_by_extension(extension).unwrap();
                        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
                        let splits = source.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
                        let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
                        let x = 40 * get_justify((screen_width() / 30.0) as usize, v2).unwrap();
                        for line in LinesWithEndings::from(source) {
                            let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
                            let mut dx = 0;
                            for range in ranges {
                                let c = range.0.foreground;
                                draw_text_ex(range.1, (x + (dx * 20)) as f32, y, TextParams { font_size: 40, font,
                                color: macroquad::color::Color::new(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0),
                                    ..Default::default()
                                });
                                dx += range.1.len();
                            }
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
        if antibounce.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() >= 200 {
            if is_key_down(miniquad::KeyCode::Right) || is_key_down(miniquad::KeyCode::Down) {
                i += 1;
                changed = true;
            }
            if is_key_down(miniquad::KeyCode::Left) || is_key_down(miniquad::KeyCode::Up) {
                i -= 1;
                changed = true;
            }
            if is_key_down(miniquad::KeyCode::Q) {
                return;
            }
            if is_key_down(miniquad::KeyCode::S) {
                let png_path = format!("bema_slide_{}.png", i);
                println!("export png: {}", png_path);
                macroquad::texture::get_screen_data().export_png(&png_path);
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
        next_frame().await;
    }
}

impl Runner for GuiRunner {
    fn run(&self, bema: &Bema) -> Result<()> {

        macroquad::Window::new("Bema", main_gui_runner(bema.clone()));
        Ok(())
    }
}


pub struct HovercraftRunner {
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
