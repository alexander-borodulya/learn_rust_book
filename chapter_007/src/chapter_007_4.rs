mod module_a {
    pub mod submodule_a {
        pub fn test_submodule_a() {
            println!("test_submodule_a");
        }
    }
}

mod module_b {
    // pub mod submodule_b {
    //     pub fn test_submodule_b() {
    //         println!("test_submodule_b about to call test_submodule_a");
            
    //         // The following code:
    //         // submodule_a::test_submodule_a();

    //         // Generates error message:
    //         // error[E0433]: failed to resolve: use of undeclared crate or module `submodule_a`
    //         //   --> src/chapter_007/chapter_007_4.rs:15:13
    //         //    |
    //         // 15 |             submodule_a::test_submodule_a();
    //         //    |             ^^^^^^^^^^^ use of undeclared crate or module `submodule_a`
    //     }
    // }

    pub mod submodule_b {
        use crate::chapter_007_4::module_a::submodule_a;
        pub fn test_submodule_b() {
            println!("test_submodule_b about to call test_submodule_a");
            submodule_a::test_submodule_a();
        }
    }
}



use std::collections::BinaryHeap;

use module_a::submodule_a;
use module_b::submodule_b;

// Test re-exporting.. Makes all public content visible in this scope (whole file scope)
use crate::chapter_007_4_reexport;

pub fn chapter_007_4_impl() {
    println!("7.4. Bringing Paths into Scope with the use Keyword");

    // 1 - Bringing Paths into Scope with the use Keyword
    {
        // 1. From the scope of entire file
        submodule_a::test_submodule_a();
    
        // 2. From the scope of a submodule_b
        submodule_b::test_submodule_b();
    }
    
    // 2 - Creating Idiomatic use Paths
    {
        // Bringing HashMap collection
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            map.insert(1, 2);
            println!("map: {:?}", map);
        }
        
        // Bringing different types of result
        {
            use std::fmt;
            use std::io;
            
            let f_fmt = || -> fmt::Result {
                let r = fmt::Result::Ok(());
                r
            };
            
            let f_io = || -> io::Result<i32> {
                let r = io::Result::Ok(100);
                r
            };
            
            println!("f_fmt: {:?}", f_fmt());
            println!("f_io: {:?}", f_io());
        }
    }

    // 3 - Providing New Names with the as Keyword
    {
        use std::fmt::Result as FmtResult;
        use std::io::Result as IoResult;
        
        let f_fmt = || -> FmtResult { FmtResult::Ok(()) };
        let f_io = || -> IoResult<i32> { IoResult::Ok(100) };
        
        println!("f_fmt: {:?}", f_fmt());
        println!("f_io: {:?}", f_io());
    }
    
    // 4 - Re-exporting Names with pub use
    {
        // Usege 1: Use full path to the function
        println!("Usage 1");
        chapter_007_4_reexport::module_c_caller::submodule_c_caller::submodule_c_caller_impl::test_submodule_c_caller();
        
        // Usage 2: The path has been shortened using re-exporting
        println!("Usage 2");
        chapter_007_4_reexport::submodule_c_caller_impl::test_submodule_c_caller();
        
        // Usage 3: Use reexported function from the chapter_007_4_reexport module
        println!("Usage 3");
        chapter_007_4_reexport::test_submodule_c_callee();
    }

    // 5 - Using External Packages
    {
        {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let rand_i32 = rng.gen_range(0..10);
            let rnad_f64 = rng.gen_range(-1.0..1.0);
            println!("rand_i32: {}", rand_i32);
            println!("rnad_f64: {}", rnad_f64);
        }

        {
            use std::collections::BTreeMap;
            use std::collections::HashMap;

            let keys = [10, 5, 20, 35, 1];
            let values = [1, 2, 3, 4, 5];

            let mut hash_map = HashMap::new();
            let mut b_tree_map = BTreeMap::new();

            for (index, key) in keys.iter().enumerate() {
                hash_map.insert(key, values[index]);
                b_tree_map.insert(key, values[index]);
            }

            println!("keys: {:?}", keys);
            println!("values: {:?}", values);
            println!("hash_map: {:?}", hash_map);
            println!("b_tree_map: {:?}", b_tree_map);
        }

        // Panic
        // thread 'main' panicked at 'cannot sample empty range'
        // let rand_i128 = rng.gen_range(1..1_i128);
    }

    // 6 - Using Nested Paths to Clean Up Large use Lists
    {
        // 1
        {
            use std::collections::{BTreeMap, HashMap};
            
            let mut btm: BTreeMap<i8, i8> = BTreeMap::default();
            btm.insert(1, 2);
            let mut hm = HashMap::new();
            hm.insert(0, 1);
        }

        // 2
        {
            #[allow(unused_imports)]
            use std::collections::{self, HashSet};

            let mut hs = HashSet::new();
            hs.insert(1);
            hs.insert(2);
            hs.insert(3);
            hs.insert(4);
            hs.insert(5);
            println!("hs: {:?}", hs);

            let mut bh = BinaryHeap::new();
            bh.push(1);
            bh.push(2);
            bh.push(3);
            bh.push(4);
            bh.push(5);
            println!("bh: {:?}", bh);
        }
    }

    // 7 - The Glob Operator
    {
        use std::collections::*;
        let mut v = Vec::new();
        let mut vd = VecDeque::new();
        let mut ll = LinkedList::new();

        v.push(1);
        vd.push_back(1);
        vd.push_front(2);
        ll.push_back(10);
        ll.push_front(20);

        println!("v: {:?}", v);
        println!("vd: {:?}", vd);
        println!("ll: {:?}", ll);
    }
}
