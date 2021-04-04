use crate::bema::Bema;

use crossterm::Result;

pub trait Runner {
    fn run(&self, bema: &Bema) -> Result<()>;
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

