mod runner;
use crate::runner::{GuiRunner, TerminalRunner, HovercraftRunner, Runner};
mod bema;
use crate::bema::{Bema, SlideItem, Slide};

use std::env;

use crossterm::Result;

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
