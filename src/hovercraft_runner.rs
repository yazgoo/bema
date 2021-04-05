use crate::runner::Runner;
use crate::bema::{Bema, SlideItem};
use std::fs::File;
use crossterm::Result;
use std::io::Write;

pub struct HovercraftRunner {
}

impl HovercraftRunner {
    fn render_item(&self, item: &SlideItem, img_i: &mut usize) -> Result<()> {
        match item {
            SlideItem::Image { image, extension, width } => {
                let file_path = format!("bema_{}{}", img_i, extension);
                *img_i += 1;
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
            SlideItem::Rows { items } => {
                for item2 in items {
                    self.render_item(item2, img_i)?;
                }
            },
        }
        Ok(())
    }
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
                self.render_item(item, &mut img_i)?;
            };
            println!("");
        }
        Ok(())
    }
}
