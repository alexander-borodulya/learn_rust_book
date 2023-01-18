#[allow(dead_code)]
pub struct Guess {
    value: u32,
}

#[allow(dead_code)]
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than 1, got: {}", value);
        } else if value > 100 {
            panic!("Guess must be less than 100, got: {}", value);
        }

        Guess { value }
    }

    pub fn new_wbug(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess must be between 1 and 100, got: {}", value);
        }
        Guess { value }
    }

    pub fn new_wbug_panic_msg(value: u32) -> Guess {
        if value < 1 {
            panic!("Must GT 1, got: {}", value);
        } else if value > 100 {
            panic!("Must LE 100, got: {}", value);
        }

        Guess { value }
    }
}

//
// pub struct Object
//

#[derive(Debug)]
pub struct Object {
    pub data: u32,
}

#[allow(dead_code)]
impl Object {
    pub fn new(data: u32) -> Object {
        if data == 0 {
            panic!("panic expected: data: {}", data);
        } else if data == 1 {
            panic!("panic un-expected: data: {}", data);
        }

        Object { data }
    }
}

//
// Result <Ok, Err>
//

#[derive(Debug, PartialEq)]
pub struct MyError {
    pub message: String,
}
