#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T = i32, U = i32> {
    pub x: T,
    pub y: U,
}

impl Point {
    pub fn works_when_i32(&self) {
        println!("works when i32: [{}, {}]", self.x, self.y);
    }
}

impl Point<f64, f64> {
    pub fn works_when_f64(&self) {
        println!("works when f64: [{}, {}]", self.x, self.y);
    }
}

impl<T, U> Point<T, U>
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    pub fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    pub fn works_when_t_and_u(&self) {
        println!("works when t and u: [{}, {}]", self.x, self.y);
    }
}

// 19.4
// Advanced Functions and Closures
pub mod subchapter_04 {
    #[derive(Debug, PartialEq)]
    pub enum States {
        Pending,
        Progress(isize),
        Result(String),
    }

    // Required by ToString::to_string
    impl std::fmt::Display for States {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                States::Pending => write!(f, "Pending"),
                States::Progress(p) => write!(f, "Progress({})", p),
                States::Result(r) => write!(f, "Result({})", r),
            }
        }
    }
}
