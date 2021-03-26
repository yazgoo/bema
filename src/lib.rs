use tempfile::Builder;
use std::io::{stdout, Write};
use std::process::Command;
use std::env;
use blockish::render_image;

use crossterm::{
    execute,
    cursor::{MoveTo, MoveRight, Hide, Show},
    event::{Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Attribute, SetAttribute},
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, self},
    ExecutableCommand, Result,
    event,
};

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

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

fn justify_center(text: &String) -> Result<()> {
    let whitespaces = get_justify(vec![text])?;
    stdout()
        .execute(MoveRight(whitespaces as u16))?;
    Ok(())
}

trait SlideItem {
    fn render(&self) -> Result<()>;
}

struct Code {
    extension: String,
    source: String,
}

impl SlideItem for Code {
    fn render(&self) -> Result<()> {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let syntax = ps.find_syntax_by_extension(&self.extension).unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
        let splits = self.source.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
        let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
        let whitespaces = get_justify(v2)? as usize;
        for line in LinesWithEndings::from(&self.source) {
            let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            stdout()
                .execute(ResetColor)?;
            for _ in 0..whitespaces { print!(" ") }
            print!("{}", escaped);
        }

        stdout()
        .execute(ResetColor)?;

        Ok(())
    }
}

struct Text {
    text: String
}

impl SlideItem for Text {
    fn render(&self) -> Result<()> {
        justify_center(&self.text)?;
        println!("{}", self.text);
        Ok(())
    }
}

struct Image {
    image: &'static [u8],
    extension: String,
}

impl SlideItem for Image {
    fn render(&self) -> Result<()> {

        let mut file = Builder::new()
            .prefix("image")
            .suffix(&self.extension)
            .rand_bytes(5)
            .tempfile()?;
        file.write(self.image)?;
        display_image(&file.path().to_str().unwrap().to_string());
        Ok(())
    }
}

pub struct Slide {
    title: String,
    items: Vec<Box<dyn SlideItem>>,
}

pub struct Bema {
    slides: Vec<Slide>
}

pub fn slides(f: fn(Bema) -> Bema) -> Bema {
    f(Bema { 
        slides: vec![],
    })
}

impl Slide {
    pub fn render(&self) -> Result<()> {

        justify_center(&self.title)?;

        stdout()
        .execute(SetAttribute(Attribute::Bold))?
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Black))?
        .execute(Print(self.title.to_string()))?
        .execute(ResetColor)?
        .execute(Print("\n\n"))?;

        for item in &self.items {
            let _ = item.render();
        }

        Ok(())
    }
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

    pub fn clear_screen(&self) -> Result<()> {

        stdout()
            .execute(Clear(ClearType::All))?;

        Ok(())
    }

    pub fn read_char(&self) -> Result<char> {
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
    pub fn run(&self) -> Result<()> {
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
                    'G' => i = self.slides.len() as i16 - 1,
                    'n'|'j'|'l' => i+=1,
                    'p'|'k'|'h' => i-=1,
                    'q' => break,
                    _ => {}
                }
                if i as usize >= self.slides.len() {
                    i = 0;
                }
                if i < 0 {
                    i = self.slides.len() as i16 - 1;
                }
            } else {
                i = 0;
            }
            execute!(
                stdout(),
                MoveTo(0, 0),
            )?;
            self.clear_screen()?;
            println!("{}/{}", i + 1, self.slides.len());
            self.slides.get(i as usize).unwrap().render()?;
        }

        execute!(
            stdout(),
            Show
        )?;

        Ok(())
    }
}

impl Slide {
    pub fn text(mut self, s: &str) -> Slide {
        self.items.push(Box::new(Text { text: String::from(s) }));
        self
    }

    pub fn t(self, s: &str) -> Slide {
        self.text(s)
    }

    pub fn code(mut self, extension: &str, source: &str) -> Slide {
        self.items.push(Box::new(Code { extension: String::from(extension), source: String::from(source) }));
        self
    }

    pub fn image(mut self, s: &'static [u8], extension: &str) -> Slide {
        self.items.push(Box::new(Image { image: s, extension: String::from(extension) }));
        self
    }
}
