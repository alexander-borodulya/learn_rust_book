// Makes modules belows visible in the scopet of the entire file
mod chapter_007_impl;
mod chapter_007_2_submodule;
mod chapter_007_4;
mod chapter_007_4_reexport;

/// Declare crate level `print_module_name` function
pub fn print_module_name() {
    println!("chapter_007.rs: print_module_name: chapter_007");
}

/// mod fron_of_house, another inlined module
#[allow(dead_code)]
pub mod fron_of_house {

    /// mod hosting
    pub mod hosting {
        /// Adds customer to a waitlist
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

/// Chapter 7.2. Declare Crate level module: Inlined
mod chapter_007_crate_level_module_v1 {
    #[allow(dead_code)]
    pub fn print_module_name() {
        println!("chapter_007_crate_level_module_v1: print_module_name");
    }
}

/// Chapter 7.2. Declare Crate level module: In the dedicated file
mod chapter_007_crate_level_module_v2;

/// Chapter 7.3. Nested function calls crate root level function
pub fn chapter_007_3_crate_root_fn() {
    println!("main.rs: chapter_007_3_crate_root_fn called...");
}

/// Chapter 7.4. Test re-export
pub use chapter_007_4_reexport::submodule_reexport::test_submodule_re;

pub fn run() {
    chapter_7_1();
    chapter_7_2();
    chapter_007_2();
    chapter_7_3();
    {
        // Creates shortcuts for the content of the declared module: chapter_007_4_impl function
        use chapter_007_4::chapter_007_4_impl;

        // Calling the chapter_007_4_impl function from the chapter_007_4 module
        chapter_007_4_impl();        
    }
}

fn chapter_7_1 () {
    println!("7.1. Packages and Crates");
    // Crates package within a binary crate
    //  > cargo new chapter_007 

    // Adds two more binary crates
    //  > mkdir -p ./chapter_007/src/bin/
    //  > echo 'fn main() { println!("chapter_007_binary_001"); }' > ./chapter_007/src/bin/binary_001.rs
    //  > echo 'fn main() { println!("chapter_007_binary_002"); }' > ./chapter_007/src/bin/binary_002.rs
    
    // Creates a package within a lib crate
    //  > cargo new chapter_007_lib --lib
}


fn chapter_7_2 () {
    println!("7.2. Defining Modules to Control Scope and Privacy");
    
    mod root_level {
        pub fn do_all_tests() {
            println!("root_level::do_all_tests");
            module_a::test();
            module_b::test();
        }

        mod module_a {
            pub fn test() {
                println!("root_level::module_a::test");
            }
        }

        mod module_b {
            pub fn test() {
                println!("root_level::module_b::test");
                module_c::test();
            }
            mod module_c {
                pub fn test() {
                    println!("root_level::module_b::module_c::test");
                }
            }
        }
    }

    root_level::do_all_tests();
}

fn chapter_007_2() {
    println!("7.2. Defining Modules to Control Scope and Privacy");

    // Access crate level inlined module
    crate::chapter_007_crate_level_module_v1::print_module_name();
    
    // Access crate level module from the crate_level_module_v2.rs file
    crate::chapter_007_crate_level_module_v2::print_module_name();
    
    // Access submodule / this module using absolute path
    crate::print_module_name();
    
    // Access submodule absolute path
    crate::chapter_007_2_submodule::print_module_name();
    
    // Access function form a root scope of the current module
    print_module_name();
    
    // Access submodule function from a scope of the current module
    chapter_007_2_submodule::print_module_name();
}

fn chapter_7_3 () {
    println!("7.3. Paths for Referring to an Item in the Module Tree");
    chapter_007_impl::apartment::lock();
    chapter_007_impl::apartment::unlock();

    chapter_007_impl::apartment::hall::test_self();
    chapter_007_impl::apartment::hall::test_super();
    chapter_007_impl::apartment::hall::test_crate();
}
