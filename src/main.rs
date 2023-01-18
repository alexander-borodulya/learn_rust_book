mod chapter_001;
mod chapter_002;
mod chapter_003;
mod chapter_004;
mod chapter_005;
mod chapter_006;
mod chapter_007;
mod chapter_008;
mod chapter_009;
mod chapter_010;
mod chapter_011;
mod chapter_012;
mod chapter_013;
mod chapter_015;
mod chapter_016;
mod chapter_017;
mod chapter_018;
mod chapter_019;
mod rust_book;

//////////////////////////////////////////////////////////////////////////////
//
// Chapter 7 - Begin
//

// Chapter 7.2: Declare Crate level module: Inlined
mod chapter_007_crate_level_module_v1 {
    #[allow(dead_code)]
    pub fn print_module_name() {
        println!("chapter_007_crate_level_module_v1: print_module_name");
    }
}

// Chapter 7.2: Declare Crate level module: In the dedicated file
mod chapter_007_crate_level_module_v2;

// Chapter 7.3: Nested function calling crate root level function
pub fn chapter_007_3_crate_root_fn() {
    println!("main.rs: chapter_007_3_crate_root_fn called...");
}

//
// Chapter 7 - End
//
//////////////////////////////////////////////////////////////////////////////

fn main() {
    // chapter_001::run();
    // chapter_002::run();
    // chapter_003::run();
    // chapter_004::run();
    // chapter_005::run();
    // chapter_006::run();
    // chapter_007::run();
    // chapter_008::run();
    // chapter_009::run();
    // chapter_010::run();
    // chapter_011::run();
    // chapter_012::run();
    // chapter_013::run();
    // chapter_015::run();
    // chapter_016::run();
    // chapter_017::run();
    // chapter_018::run();
    chapter_019::run();
}
