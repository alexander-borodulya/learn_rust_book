#[derive(Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }
}

#[derive(Debug)]
pub struct Point4 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub w: isize,
}

#[allow(dead_code)]
impl Point4 {
    pub fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self { x, y, z, w }
    }
}
