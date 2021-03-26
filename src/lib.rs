use std::io::{stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Attribute, SetAttribute},
    ExecutableCommand, Result,
    event,
};

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

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
        for line in LinesWithEndings::from(&self.source) {
            let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            println!("{}", escaped);
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
        println!("{}", self.text);
        Ok(())
    }
}

struct Image {
    image: String
}

impl SlideItem for Image {
    fn render(&self) -> Result<()> {
        println!("{}", self.image);
        Ok(())
    }
}

struct Diagram {
    diagram: String,
}

impl SlideItem for Diagram {
    fn render(&self) -> Result<()> {
        println!("{}", self.diagram);
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

        stdout()
        .execute(SetAttribute(Attribute::Bold))?
        .execute(SetForegroundColor(Color::Blue))?
        .execute(SetBackgroundColor(Color::Black))?
        .execute(Print(self.title.to_string()))?
        .execute(ResetColor)?
        .execute(Print("\n\n"))?;

        for item in &self.items {
            item.render();
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
    pub fn run(&self) {
        for slide in &self.slides {
            slide.render();
        }
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

    pub fn diagram(mut self, s: &str) -> Slide {
        self.items.push(Box::new(Diagram { diagram: String::from(s) }));
        self
    }

    pub fn image(mut self, s: &str) -> Slide {
        self.items.push(Box::new(Image { image: String::from(s) }));
        self
    }
}
