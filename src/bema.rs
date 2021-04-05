
#[derive(Clone)]
pub enum SlideItem {
    Code{ extension: String, source: String },
    Image{ image: Vec<u8>, extension: String, width: Option<usize> },
    Text{ text: String },
    Rows { items: Vec<Box<SlideItem>> },
}

#[derive(Clone)]
pub struct Slide {
    pub title: String,
    pub items: Vec<SlideItem>,
    pub vertical_count: usize,
    pub current_slideitems: Vec<SlideItem>,
}

#[derive(Clone)]
pub struct Bema {
    pub slides: Vec<Slide>
}
