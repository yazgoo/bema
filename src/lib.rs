mod runner;
use crate::runner::{Runner};
mod hovercraft_runner;
use crate::hovercraft_runner::HovercraftRunner;
mod terminal_runner;
use crate::terminal_runner::TerminalRunner;
mod gui_runner;
use crate::gui_runner::GuiRunner;
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

pub struct SlideItems {
    items: Vec<SlideItem>,
}

impl SlideItems {
    pub fn push(mut self, item: SlideItem) -> SlideItems {
        self.items.push(item);
        self
    }

    pub fn text(self, s: &str) -> SlideItems {
        self.push(SlideItem::Text { text: String::from(s) })
    }

    pub fn t(self, s: &str) -> SlideItems {
        self.text(s)
    }

    pub fn code(self, extension: &str, source: &str) -> SlideItems {
        self.push(SlideItem::Code { extension: String::from(extension), source: String::from(source) })
    }

    pub fn image(self, image: Vec<u8>, extension: &str, width: Option<usize>) -> SlideItems {
        self.push(SlideItem::Image { image, extension: String::from(extension), width })
    }

    pub fn cols(self, f: fn(SlideItems) -> SlideItems) -> SlideItems {
        self.push(SlideItem::Cols { items: f(SlideItems { items: vec![]}).items })
    }

    pub fn rows(self, f: fn(SlideItems) -> SlideItems) -> SlideItems {
        self.push(SlideItem::Rows { items: f(SlideItems { items: vec![]}).items })
    }

    pub fn framed(self, f: fn(SlideItems) -> SlideItems) -> SlideItems {
        self.push(SlideItem::Framed { items: f(SlideItems { items: vec![]}).items })
    }
}

impl Slide {
    pub fn push(mut self, item: SlideItem) -> Slide {
        self.items.push(item);
        self
    }

    pub fn text(self, s: &str) -> Slide {
        self.push(SlideItem::Text { text: String::from(s) })
    }

    pub fn t(self, s: &str) -> Slide {
        self.text(s)
    }

    pub fn code(self, extension: &str, source: &str) -> Slide {
        self.push(SlideItem::Code { extension: String::from(extension), source: String::from(source) })
    }

    pub fn image(self, image: Vec<u8>, extension: &str, width: Option<usize>) -> Slide {
        self.push(SlideItem::Image { image, extension: String::from(extension), width })
    }

    pub fn cols(self, f: fn(SlideItems) -> SlideItems) -> Slide {
        self.push(SlideItem::Cols { items: f(SlideItems { items: vec![]}).items })
    }

    pub fn rows(self, f: fn(SlideItems) -> SlideItems) -> Slide {
        self.push(SlideItem::Rows { items: f(SlideItems { items: vec![]}).items })
    }

    pub fn framed(self, f: fn(SlideItems) -> SlideItems) -> Slide {
        self.push(SlideItem::Framed { items: f(SlideItems { items: vec![]}).items })
    }
}
