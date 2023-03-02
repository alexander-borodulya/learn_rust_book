#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn can_hold_wbug(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}
