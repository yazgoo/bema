use tempfile::Builder;
use std::io::{stdout, Write};
use std::process::Command;
use std::fs::File;
use std::env;
use blockish::render_image;

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

enum SlideItem {
    Code{ extension: String, source: String },
    Image{ image: &'static [u8], extension: String, width: Option<usize> },
    Text{ text: String },
}

pub struct Slide {
    title: String,
    items: Vec<SlideItem>,
}

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

fn get_justify(texts: Vec<&String>) -> Result<usize> {

    let size = terminal::size()?;

    let mut whitespaces : usize = size.0 as usize;

    for text in texts {
        let new_whitespaces = if text.len() < size.0 as usize {
            let x = (size.0 as usize - text.len()) / 2;
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

fn justify_center(text: Vec<&String>) -> Result<()> {
    let whitespaces = get_justify(text)?;
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

        justify_center(vec![&slide.title])?;

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
                    let whitespaces = get_justify(v2)? as usize;
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
                    let whitespaces = get_justify(v2)?;
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
            if args[1] == "hovercraft" { HovercraftRunner { }.run(&self)? }
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