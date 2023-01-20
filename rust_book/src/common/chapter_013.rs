#[derive(Debug, PartialEq)]
pub struct Shoe {
    pub size: i32,
    pub style: String,
}

#[allow(dead_code)]
impl Shoe {
    pub fn new(size: i32, style: String) -> Shoe {
        Shoe {
            size,
            style,
        }
    }

    pub fn shoes_by_size(
        shoes: Vec<Shoe>, 
        size: i32
    ) -> Vec<Shoe> {
        shoes
            .into_iter()
            .filter(|s| s.size == size)
            .collect()
    }
}
