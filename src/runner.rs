use crate::bema::Bema;
use image::io::Reader as ImageReader;

use crossterm::Result;

pub trait Runner {
    fn run(&self, bema: &Bema) -> Result<()>;
}

pub fn fit_image_bytes(bytes: &[u8], width: &Option<usize>, extension: &String) -> Vec<u8> {
    let mut img = ImageReader::with_format(std::io::Cursor::new(bytes), image::ImageFormat::from_extension(extension.replace(".", "")).unwrap()).decode().unwrap();
    img = width.map(|w| img.resize(w as u32, (w * 2) as u32, image::imageops::FilterType::Lanczos3)).unwrap_or(img);
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut bytes, image::ImageOutputFormat::Png).unwrap();
    bytes
}


pub fn get_justify(size: usize, texts: Vec<&String>) -> Result<usize> {

    let mut whitespaces : usize = size as usize;

    for text in texts {
        let new_whitespaces = if text.len() < size as usize {
            let x = (size as usize - text.len()) / 2;
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

