//
// For re-exporting
//
mod module_c_callee {
    pub mod submodule_c_callee {
        pub fn test_submodule_c_callee() {
            println!("module_c_callee: submodule_c_callee: test_submodule_c_callee");
        }
    }
}

// Define module, submodules and function
pub mod module_c_caller {
    pub mod submodule_c_caller {
        pub mod submodule_c_caller_impl {
            pub fn test_submodule_c_caller() {
                println!("module_c_caller: submodule_c_caller: submodule_c_caller_impl: test_submodule_c_caller: ---> Reexported function");
            }
        }
    }
}

// Reexporting module submodule_c_caller_impl...
pub use crate::chapter_007_4_reexport::module_c_caller::submodule_c_caller::submodule_c_caller_impl;

// Reexporting specific function...
pub use self::module_c_callee::submodule_c_callee::test_submodule_c_callee;

pub mod submodule_reexport {
    pub fn test_submodule_re() {
        println!("module_ch_7_4_reexport: submodule_reexport: test_submodule_re");
    }
}
