use crate::runner::{Runner, get_justify};

use crate::bema::{Bema, SlideItem, Slide};
use tempfile::Builder;
use std::io::{stdout, Write};
use std::process::Command;
use std::env;
use blockish::render_image;

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crossterm::{
    execute,
    cursor::{MoveTo, MoveRight, Hide, Show},
    event::{Event, KeyCode, KeyEvent},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Attribute, SetAttribute},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType, enable_raw_mode, disable_raw_mode, self},
    ExecutableCommand, Result,
    event,
};


fn justify_center(size: usize, text: Vec<&String>) -> Result<()> {
    let whitespaces = get_justify(size, text)?;
    stdout()
        .execute(MoveRight(whitespaces as u16))?;
    Ok(())
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

    fn render_item(&self, item: &SlideItem) -> Result<()> {
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
                SlideItem::Cols { items } => {
                    for item2 in items {
                        self.render_item(item2)?;
                    }
                },
                SlideItem::Rows { items } => {
                    for item2 in items {
                        self.render_item(item2)?;
                    }
                },
        }
        Ok(())
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
            self.render_item(item)?;
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
