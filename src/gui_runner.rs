use crate::runner::{Runner, get_justify};
use crate::bema::{Bema, SlideItem, Slide};

use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::LinesWithEndings;

use crossterm::Result;
use macroquad::prelude::*;

pub struct GuiRunner {
}


fn get_justify_px(font_size: u16, texts: Vec<&String>) -> f32 {
    let font_width = font_size / 2;
    (font_width as usize * get_justify((screen_width() / font_width as f32) as usize, texts).unwrap_or(0)) as f32
}

fn main_draw_texture(textures: &mut HashMap<(i32, usize),Texture2D>, bytes: &[u8], width: &Option<usize>, extension: &String, pos: usize, i: i32, y: &mut f32) {
    match textures.get(&(i, pos)) {
        Some(_) => {},
        None => {
            let quad_context = unsafe { get_internal_gl() }.quad_context;
            let texture = if extension == ".jpg" {
                let mut img = ImageReader::with_format(std::io::Cursor::new(bytes), image::ImageFormat::Jpeg).decode().unwrap();
                img = width.map(|w| img.resize(w as u32, (w * 2) as u32, image::imageops::FilterType::Lanczos3)).unwrap_or(img);
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
    draw_texture(texture, x, *y, WHITE);
    *y += texture.width();
}

fn main_capture_input(bema: &Bema, i: &mut i32, scale: &mut f32, slide: &mut Slide, antibounce: &mut SystemTime) {
    let mut changed = false;

    if antibounce.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() >= 200 {
        if is_key_down(miniquad::KeyCode::Right) || is_key_down(miniquad::KeyCode::Down) || is_key_down(miniquad::KeyCode::L) || is_key_down(miniquad::KeyCode::J) || is_key_down(miniquad::KeyCode::N) {
            *i += 1;
            changed = true;
        }
        if is_key_down(miniquad::KeyCode::Left) || is_key_down(miniquad::KeyCode::Up) || is_key_down(miniquad::KeyCode::H) || is_key_down(miniquad::KeyCode::K) || is_key_down(miniquad::KeyCode::P) {
            *i -= 1;
            changed = true;
        }
        if is_key_down(miniquad::KeyCode::Q) {
            std::process::exit(0);
        }
        if is_key_down(miniquad::KeyCode::M) {
            *scale *= 1.1;
        }
        if is_key_down(miniquad::KeyCode::R) {
            *scale /= 1.1;
        }
        if is_key_down(miniquad::KeyCode::S) {
            let png_path = format!("bema_slide_{}.png", *i);
            println!("export png: {}", png_path);
            macroquad::texture::get_screen_data().export_png(&png_path);
        }
        if *i >= bema.slides.len() as i32 {
            *i = 0;
        }
        else if *i < 0 {
            *i = bema.slides.len() as i32 - 1;
        }
        if changed {
            *slide = bema.slides.get(*i as usize).unwrap().clone();
        }
        *antibounce = SystemTime::now();
    }
}

fn scalef(font_size: u16, scale: f32) -> u16 {
    (font_size as f32 * scale as f32) as u16
}

fn write_text(text_size: u16, font: Font, y: &mut f32, text: &String) {
    let splits = text.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
    let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
    let x = get_justify_px(text_size, v2);
    for split in splits {
        draw_text_ex(&split, x, *y, TextParams { font_size: text_size, font,
            ..Default::default()
        });
        *y += text_size as f32;
    }
}

fn write_code(text_size: u16, font: Font, y: &mut f32, extension: &String, source: &String) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(extension).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let splits = source.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
    let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
    let x = get_justify_px(text_size, v2);
    for line in LinesWithEndings::from(source) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let mut dx = 0.0;
        for range in ranges {
            let c = range.0.foreground;
            draw_text_ex(range.1, (x + (dx * (text_size as f32 / 2.0))) as f32, *y, TextParams { font_size: text_size, font,
            color: macroquad::color::Color::new(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0),
            ..Default::default()
            });
            dx += range.1.len() as f32;
        }
        *y += text_size as f32;
    }

}

async  fn main_gui_runner(bema: Bema) {
    let font = load_ttf_font_from_bytes(include_bytes!("3270 Narrow Nerd Font Complete.ttf"));
    let mut i : i32 = 0;
    let mut slide : Slide = bema.slides.get(i as usize).unwrap().clone();
    let mut antibounce = SystemTime::now(); 
    let mut textures = HashMap::new();

    let mut scale : f32 = 1.0;

    loop {
        let title_size : u16 = scalef(80, scale);
        let text_size : u16 = scalef(60, scale);
        let index_size : u16 = scalef(20, scale);
        clear_background(BLACK);
        
        let mut y = index_size as f32;
        draw_text_ex(format!("{}/{}", i + 1, bema.slides.len()).as_str(), 20.0, y, TextParams { font_size: index_size, font,
                                            ..Default::default()
                                                            });
        y += title_size as f32;

        draw_text_ex(&slide.title, get_justify_px(title_size, vec![&slide.title]), y, TextParams { font_size: title_size, font,
                                            ..Default::default()
                                                            });
        y += 2.0 * title_size as f32;
            for (pos, item) in slide.items.iter().enumerate() {
                match item {
                    SlideItem::Image { image: bytes, extension, width } => {
                        main_draw_texture(&mut textures, bytes, width, &extension, pos, i, &mut y);
                    },
                    SlideItem::Code { extension, source } => {
                        write_code(text_size, font, &mut y, extension, source);
                    },
                    SlideItem::Text { text } => {
                        write_text(text_size, font, &mut y, text);
                    },
                }
            };
        main_capture_input(&bema, &mut i, &mut scale, &mut slide, &mut antibounce); 
        next_frame().await;
    }
}

impl Runner for GuiRunner {
    fn run(&self, bema: &Bema) -> Result<()> {

        macroquad::Window::new("Bema", main_gui_runner(bema.clone()));
        Ok(())
    }
}

