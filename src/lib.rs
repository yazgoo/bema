trait SlideItem {
}

struct Code {
    extension: String,
    source: String,
}

impl SlideItem for Code {
}

struct Text {
    text: String
}

impl SlideItem for Text {
}

struct Image {
    image: String
}

impl SlideItem for Image {
}

struct Diagram {
    diagram: String,
}

impl SlideItem for Diagram {
}

pub struct Slide {
    title: String,
    items: Vec<Box<dyn SlideItem>>,
}

pub struct Bema {
    slides: Vec<Slide>
}

pub fn slides(f: fn(Bema) -> ()) {
    let mut bema = Bema { 
        slides: vec![],
    };
    f(bema);
}

impl Bema {
    pub fn slide(&mut self, title: &str, f: fn(Slide) -> Slide) {
        let mut s = Slide {
            title: String::from(title),
            items: vec![],
        };
        self.slides.push(f(s));
    }
}

impl Slide {
    pub fn text(&mut self, s: &str) {
        self.items.push(Box::new(Text { text: String::from(s) }));
    }

    pub fn t(&mut self, s: &str) {
        self.text(s);
    }

    pub fn code(&mut self, extension: &str, source: &str) {
        self.items.push(Box::new(Code { extension: String::from(extension), source: String::from(source) }));
    }

    pub fn diagram(&mut self, s: &str) {
        self.items.push(Box::new(Diagram { diagram: String::from(s) }));
    }

    pub fn image(&mut self, s: &str) {
        self.items.push(Box::new(Image { image: String::from(s) }));
    }
}
