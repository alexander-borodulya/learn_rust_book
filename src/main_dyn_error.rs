use std::error::Error;

mod chapter_009;

// The main function that returns any kind of error
fn main() -> Result<(), Box<dyn Error>> {
    chapter_009::run_chapter_009_2()
}
