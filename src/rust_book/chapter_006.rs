#![allow(dead_code)]

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(i32, i32, i32),
}

impl Message {
    pub fn print(&self) {
        println!("Message: {:?}", self);
    }
}

///////////////////////////////////////////////////////////////////////////
/// 
#[derive(Debug)]
pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[derive(Debug)]
pub enum MessageV2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
