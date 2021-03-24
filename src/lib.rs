pub fn slides(f: fn() -> ()) {
    f();
}

pub fn slide(title: &str, f: fn() -> ()) {
    f();
}

pub fn text(s: &str) {
}

pub fn t(s: &str) {
}

pub fn code(extension: &str, source: &str) {
}

pub fn diagram(s: &str) {
}

pub fn image(s: &str) {
}
