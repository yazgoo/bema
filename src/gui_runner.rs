use crate::runner::{Runner, get_justify};
use crate::bema::{Bema, SlideItem, Slide};
use indoc::indoc;

use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::LinesWithEndings;

use crossterm::Result;
use macroquad::prelude::*;
use miniquad::{BlendState, BlendValue, BlendFactor, Equation};

fn get_transition_duration() -> u128 {
    200
}

pub struct GuiRunner {
}


fn get_justify_px(font_size: u16, texts: Vec<&String>, total_width: f32) -> f32 {
    let font_width = font_size / 2;
    (font_width as usize * get_justify((total_width/ font_width as f32) as usize, texts).unwrap_or(0)) as f32
}

fn main_draw_texture(textures: &mut HashMap<(i32, usize),Texture2D>, bytes: &[u8], width: &Option<usize>, extension: &String, pos: usize, i: i32, dx: f32, y: &mut f32, total_width: f32) {
    match textures.get(&(i, pos)) {
        Some(_) => {},
        None => {
            let texture = if extension == ".jpg" {
                let mut img = ImageReader::with_format(std::io::Cursor::new(bytes), image::ImageFormat::Jpeg).decode().unwrap();
                img = width.map(|w| img.resize(w as u32, (w * 2) as u32, image::imageops::FilterType::Lanczos3)).unwrap_or(img);
                let mut bytes: Vec<u8> = Vec::new();
                img.write_to(&mut bytes, image::ImageOutputFormat::Png).unwrap();
                Texture2D::from_file_with_format(&bytes[..], None)
            } else {
                Texture2D::from_file_with_format(&bytes[..], None)
            };
            textures.insert((i, pos), texture);
        }
    };
    let texture = *textures.get(&(i, pos)).unwrap();
    let w = total_width;
    let x = if w < texture.width() {
        0.0
    } else {
        (w - texture.width()) / 2.0
    };
    draw_texture(texture, x + dx, *y, WHITE);
    *y += texture.width();
}

fn main_capture_input(bema: &Bema, i: &mut i32, scale: &mut f32, antibounce: &mut SystemTime, transition: &mut SystemTime, transition_direction: &mut f32, help: &mut bool, decoration: &mut bool, white_mode: &mut bool) {
    let mut changed = false;

    if antibounce.elapsed().unwrap_or(Duration::from_millis(0)).as_millis() >= get_transition_duration() {
        if is_key_down(miniquad::KeyCode::Right) || is_key_down(miniquad::KeyCode::Down) || is_key_down(miniquad::KeyCode::L) || is_key_down(miniquad::KeyCode::J) || is_key_down(miniquad::KeyCode::N) || is_key_down(miniquad::KeyCode::Space) || is_mouse_button_down(miniquad::MouseButton::Left) {
            *i += 1;
            *transition_direction = -1.0;
            changed = true;
        }
        if is_key_down(miniquad::KeyCode::Left) || is_key_down(miniquad::KeyCode::Up) || is_key_down(miniquad::KeyCode::H) || is_key_down(miniquad::KeyCode::K) || is_key_down(miniquad::KeyCode::P) || is_mouse_button_down(miniquad::MouseButton::Right) {
            *i -= 1;
            *transition_direction = 1.0;
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
        if is_key_down(miniquad::KeyCode::Escape) {
            *help = !*help;
        }
        if is_key_down(miniquad::KeyCode::D) {
            *decoration = !*decoration;
        }
        if is_key_down(miniquad::KeyCode::C) {
            *white_mode = !*white_mode;
        }
        if is_key_down(miniquad::KeyCode::S) {
            let png_path = format!("bema_slide_{}.png", *i);
            println!("export png: {}", png_path);
            macroquad::texture::get_screen_data().export_png(&png_path);
        }
        if is_key_down(miniquad::KeyCode::G) {
            *i = 0;
        }
        if *i >= bema.slides.len() as i32 {
            *i = 0;
        }
        else if *i < 0 {
            *i = bema.slides.len() as i32 - 1;
        }
        *antibounce = SystemTime::now();
        if changed {
            *transition = SystemTime::now();
            *help = false;
        }
    }
}

fn scalef(font_size: u16, scale: f32) -> u16 {
    (font_size as f32 * scale as f32) as u16
}

fn write_text(text_size: u16, font: Font, font_color: Color, dx: f32, y: &mut f32, text: &String, total_width: f32) {
    let splits = text.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
    let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
    let x = get_justify_px(text_size, v2, total_width) + dx;
    for split in splits {
        draw_text_ex(&split, x, *y, TextParams { font_size: text_size, font,
            color: font_color,
            ..Default::default()
        });
        *y += text_size as f32;
    }
}

fn write_code(text_size: u16, font: Font, dx: f32, y: &mut f32, extension: &String, source: &String, total_width: f32) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(extension).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let splits = source.split("\n").map( |x| x.to_string()).collect::<Vec<_>>();
    let v2: Vec<&String> = splits.iter().map(|s| s).collect::<Vec<&String>>();
    let x = get_justify_px(text_size, v2, total_width) + dx;
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

fn draw_item(font: Font, font_color: Color, i: i32, pos: usize, item: &SlideItem, dx: f32, y: &mut f32, total_width: f32, textures: &mut HashMap<(i32, usize), Texture2D>, scale: f32) {
    let text_size : u16 = scalef(60, scale);
    match item {
        SlideItem::Image { image: bytes, extension, width } => {
            main_draw_texture(textures, bytes, width, &extension, pos, i, dx, y, total_width);
        },
        SlideItem::Code { extension, source } => {
            write_code(text_size, font, dx, y, extension, source, total_width);
        },
        SlideItem::Text { text } => {
            write_text(text_size, font, font_color, dx, y, text, total_width);
        },
        SlideItem::Rows { items } => {
            let w = total_width / items.len() as f32;
            let mut ys = vec![];
            for (pos2, item2) in items.iter().enumerate() {
                let mut y2 = *y;
                draw_item(font, font_color, i, pos, item2, dx + w * pos2 as f32, &mut y2, w, textures, scale);
                ys.push(y2);
            }
            *y = ys.iter().cloned().fold(0.0, |a, b| { a.max(b) })
        },
    }
}

fn draw_slide(font: Font, font_color: Color, bar_color: Color, textures: &mut HashMap<(i32, usize), Texture2D>, bema: &Bema, i: i32, dx: f32, scale: f32, total_width: f32) {
    let title_size : u16 = scalef(80, scale);
    let index_size : u16 = scalef(20, scale);

    let k = if i >= (bema.slides.len() as i32) { 0 } else if i < 0 { bema.slides.len() as i32 - 1 } else { i };
    let slide = bema.slides.get(k as usize).unwrap();
    let mut y = index_size as f32;
    draw_rectangle(dx, 0.0, total_width * ((i as f32 + 1.0) / bema.slides.len() as f32), index_size as f32 / 10.0, bar_color); 
    draw_text_ex(format!("{}/{}", i + 1, bema.slides.len()).as_str(), 20.0 + dx, y, TextParams { font_size: index_size, font,
    color: bar_color,
    ..Default::default()
    });
    y += title_size as f32;

    draw_text_ex(&slide.title, get_justify_px(title_size, vec![&slide.title], total_width) + dx, y, TextParams { font_size: title_size, font,
    color: font_color,
    ..Default::default()
    });
    y += 2.0 * title_size as f32;
    for (pos, item) in slide.items.iter().enumerate() {
        draw_item(font, font_color, i, pos, item, dx, &mut y, total_width, textures, scale);
    };
}


fn draw_help(font: Font, font_color: Color, bar_color: Color, textures: &mut HashMap<(i32, usize), Texture2D>, decoration: bool, white_mode: bool, scale: f32) {
            draw_slide(font, font_color, bar_color, textures, &Bema { 
        slides: vec![Slide { 
            title: "bema help".to_string(), 
            items: vec![
                SlideItem::Text { text: "keys:".to_string() },
                SlideItem::Text { text: "".to_string() },
                SlideItem::Text { text: format!(indoc! {"
                next slide      right, down, L, J, N
                previous slide  left, up, H, K, P
                exit            Q
                scale up        M
                scale down      R
                screenshot      S
           [{}]  decoration      D 
           [{}]  white mode      C 
           [{}]  help            Escape"
                }, if decoration { "x" } else { " " }, if white_mode { "x" } else { " " }, "x") },
            ],
            vertical_count: 0,
            current_slideitems: vec![],
        }]
    }, 0, 0.0, scale, screen_width());
}

async  fn main_gui_runner(bema: Bema) {
    let font = load_ttf_font_from_bytes(include_bytes!("3270 Narrow Nerd Font Complete.ttf"));
    let mut i : i32 = 0;
    let mut antibounce = SystemTime::now(); 
    let mut transition = SystemTime::now(); 
    let mut textures = HashMap::new();

    let mut transition_direction = 0.0;
    let mut scale : f32 = 1.0;

    let mut help = false;
    let mut decoration = true;
    let mut white_mode = false;

    let render_target = render_target(screen_width() as u32, (screen_height() * 0.6) as u32);
    let material =
        load_material(CRT_VERTEX_SHADER, CRT_FRAGMENT_SHADER, Default::default()).unwrap();

    let pipeline_params = PipelineParams {
        color_blend: Some(BlendState::new(
                             Equation::Add,
                             BlendFactor::Value(BlendValue::SourceAlpha),
                             BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                     )),
                     ..Default::default()
    };

    let  reverse_material = load_material(
        CRT_VERTEX_SHADER,
        CRT_FRAGMENT_SHADER_REVERSE_BLACK,
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
        .unwrap();


    let mut font_color;
    let mut background_color;
    let mut bar_color;

    loop {
        if white_mode {
            font_color = BLACK;
            background_color = WHITE;
            bar_color = LIGHTGRAY;
        }
        else {
            font_color = WHITE;
            background_color = BLACK;
            bar_color = DARKGRAY;
        }
        if decoration {
            // draw to texture
            let camera = Camera2D {
                zoom: vec2(2.0 / screen_width(), 3.0 / screen_height()),
                target: vec2(screen_width() / 2.0, screen_height() / 3.0),
                render_target: Some(render_target),
                ..Default::default()
            };
            set_camera(&camera);
        }
        clear_background(background_color);

        if help {
            draw_help(font, font_color, bar_color, &mut textures, decoration, white_mode, scale);
        }
        else {
            let dt = transition.elapsed().unwrap_or(Duration::from_millis(0)).as_millis();
            let dt = if dt > get_transition_duration() || transition_direction == 0.0 { transition_direction = 0.0; get_transition_duration() } else { dt };
            let dx = transition_direction * screen_width() * dt as f32 / get_transition_duration() as f32;
            if transition_direction != 0.0 { draw_slide(font, font_color, bar_color, &mut textures, &bema, i - 1 + transition_direction as i32, dx - screen_width(), scale, screen_width()); }

            draw_slide(font, font_color, bar_color, &mut textures, &bema, i + transition_direction as i32, dx, scale, screen_width());
            if transition_direction != 0.0 { draw_slide(font, font_color, bar_color, &mut textures, &bema, i + 1 + transition_direction as i32, dx + screen_width(), scale, screen_width()); }
        }


        // draw to screen
        if decoration {
            set_default_camera();

            clear_background(background_color);
            gl_use_material(material);
            draw_texture_ex(
                render_target.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height() * 0.6)),
                    ..Default::default()
                },
            );
            gl_use_material(reverse_material);
            draw_texture_ex(
                render_target.texture,
                0.0,
                screen_height() * 0.6,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height() * 0.6)),
                    ..Default::default()
                },
            );
            gl_use_default_material();
        }
        main_capture_input(&bema, &mut i, &mut scale, &mut antibounce, &mut transition, &mut transition_direction, &mut help, &mut decoration, &mut white_mode); 
        next_frame().await;
    }
}

impl Runner for GuiRunner {
    fn run(&self, bema: &Bema) -> Result<()> {

        macroquad::Window::new("Bema", main_gui_runner(bema.clone()));
        Ok(())
    }
}


const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;
varying vec4 color;
varying vec2 uv;
uniform sampler2D Texture;
void main() {
    
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    gl_FragColor = vec4(res, 1.0);
}
"#;


const CRT_FRAGMENT_SHADER_REVERSE_BLACK: &'static str = r#"#version 100
precision lowp float;
varying vec4 color;
varying vec2 uv;
uniform sampler2D Texture;
uniform vec4 _Time;
void main() {
    
    vec2 uv2 = vec2(uv[0] + 0.003 * uv[1] * sin(mod(_Time.x, 100.0) + 100.0 * uv[1]), 1.0 - uv[1]); 
    vec3 res = texture2D(Texture, uv2).rgb * color.rgb;
    gl_FragColor = vec4(res, 1.0 * pow(uv2[1], 4.0));
}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying lowp vec2 uv;
varying lowp vec4 color;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";
