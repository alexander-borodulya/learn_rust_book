use std::error::Error;

// The main function that returns any kind of error
fn main() -> Result<(), Box<dyn Error>> {
    rust_book::chapter_009_2()
}
