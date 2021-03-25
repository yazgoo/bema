trait SlideItem {
    fn render(&self);
}

struct Code {
    extension: String,
    source: String,
}

impl SlideItem for Code {
    fn render(&self) {
        println!("{}", self.extension);
        println!("{}", self.source);
    }
}

struct Text {
    text: String
}

impl SlideItem for Text {
    fn render(&self) {
        println!("{}", self.text);
    }
}

struct Image {
    image: String
}

impl SlideItem for Image {
    fn render(&self) {
        println!("{}", self.image);
    }
}

struct Diagram {
    diagram: String,
}

impl SlideItem for Diagram {
    fn render(&self) {
        println!("{}", self.diagram);
    }
}

pub struct Slide {
    title: String,
    items: Vec<Box<dyn SlideItem>>,
}

pub struct Bema {
    slides: Vec<Slide>
}

pub fn slides(f: fn(Bema) -> Bema) -> Bema {
    f(Bema { 
        slides: vec![],
    })
}

impl Slide {
    pub fn render(&self) {
        println!("{}", self.title);
        for item in &self.items {
            item.render();
        }
    }
}

impl Bema {
    pub fn slide(&mut self, title: &str, f: fn(Slide) -> Slide) {
        let mut s = Slide {
            title: String::from(title),
            items: vec![],
        };
        self.slides.push(f(s));
    }
    pub fn run(&self) {
        for slide in &self.slides {
            slide.render();
        }
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
