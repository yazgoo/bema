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

pub trait Helper {
    fn push(self, item: SlideItem) -> Self where Self: Sized;

    fn text(self, s: &str) -> Self where Self: Sized {
        self.push(SlideItem::Text { text: String::from(s) })
    }

    fn t(self, s: &str) -> Self where Self: Sized {
        self.text(s)
    }

    fn code(self, extension: &str, source: &str) -> Self where Self: Sized {
        self.push(SlideItem::Code { extension: String::from(extension), source: String::from(source) })
    }

    fn image(self, image: Vec<u8>, extension: &str, width: Option<usize>) -> Self where Self: Sized {
        self.push(SlideItem::Image { image, extension: String::from(extension), width })
    }

    fn cols(self, f: fn(SlideItems) -> SlideItems) -> Self where Self: Sized {
        self.push(SlideItem::Cols { items: f(SlideItems { items: vec![]}).items })
    }

    fn rows(self, f: fn(SlideItems) -> SlideItems) -> Self where Self: Sized {
        self.push(SlideItem::Rows { items: f(SlideItems { items: vec![]}).items })
    }

    fn framed(self, f: fn(SlideItems) -> SlideItems) -> Self where Self: Sized {
        self.push(SlideItem::Framed { items: f(SlideItems { items: vec![]}).items })
    }
}

impl Helper for SlideItems {
    fn push(mut self, item: SlideItem) -> Self where Self: Sized {
        self.items.push(item);
        self
    }
}

impl Helper for Slide {
    fn push(mut self, item: SlideItem) -> Slide {
        self.items.push(item);
        self
    }
}
