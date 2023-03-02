#![allow(dead_code)]

pub trait Draw: std::fmt::Debug {
    fn draw(&self);
}

// impl std::fmt::Debug for dyn Draw {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("dyn Draw").finish()
//         // write!(f, "Trait :: dyn Draw")
//     }
// }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new(components: Vec<Box<dyn Draw>>) -> Self {
        Self { components }
    }

    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    width: usize,
    height: usize,
    label: String,
}

impl Button {
    pub fn new() -> Self {
        Self::new_with_args(100, 32, "Button".to_owned())
    }

    pub fn new_with_args(width: usize, height: usize, label: String) -> Button {
        Button {
            width, height, label,
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button: draw: {:?}", self);
    }
}

#[derive(Debug)]
pub struct Label {
    width: usize,
    height: usize,
    text: String,
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}

impl Label {
    pub fn new() -> Self {
        Self {
            width: 80,
            height: 30,
            text: "Label".to_owned(),
        }
    }
}

impl Draw for Label {
    fn draw(&self) {
        println!("Label: draw: {:?}", self);
    }
}

// Implementation of the Draw trait for the String type
impl Draw for String {
    fn draw(&self) {
        println!("String: draw: {:?}", self);
    }
}
