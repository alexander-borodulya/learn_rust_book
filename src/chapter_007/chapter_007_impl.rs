pub mod apartment {

    pub fn lock() {
        println!("locked...");
    }
    pub fn unlock() {
        println!("unlocked...");
    }

    pub fn summary() {
        hall::info();
        kitchen::info();
        wc::info();
        bedroom::info();
        balcony::info();
        terrace::info();
    }

    pub fn info() {
        println!("apartment: info:");
    }

    pub mod hall {
        pub fn store_things() {
            println!("store_things...");
        }

        pub fn info() {
            println!("hall: info:")
        }

        pub fn test_self() {
            self::info();
        }

        pub fn test_super() {
            super::info();
        }

        pub fn test_crate() {
            crate::chapter_007_3_crate_root_fn();
        }
    }
    mod kitchen {
        pub fn boil_water() {
            println!("boil_water...");
        }

        pub fn info() {
            println!("kitchen info")
        }
    }
    mod wc {
        pub fn wash_hands() {
            println!("wash_hands...");
        }

        pub fn info() {
            println!("wc info")
        }
    }
    mod bedroom {
        pub fn watch_tv() {
            println!("watch_tv...");
        }

        pub fn info() {
            println!("bedroom info")
        }
    }
    mod balcony {
        pub fn drink_coffee() {
            println!("drink_coffee...");
        }

        pub fn info() {
            println!("balcony info")
        }
    }
    mod terrace {
        pub fn chill() {
            println!("chill...");
        }

        pub fn info() {
            println!("terrace info")
        }
    }
}
