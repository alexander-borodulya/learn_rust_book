use add_one;
use sub_one;

use rand::Rng;
use std::ops::RangeInclusive;

/// adder entry point - fun function (i.e. main function)
pub fn run(_subchapter_index: u32) {
    let i = 100;
    let ai = add_one::add_one(i);
    println!("add_one: {i} + 1 = {}", ai);

    let si = sub_one::sub_one(i);
    println!("sub_one: {i} - 1 = {}", si);

    let r = add_one::get_rand_from_range(50, 150);
    println!("r = {}", r);

    let from = 2.5;
    let to = 30.0;
    let f = get_rand_from_range(from, to);
    println!("f = {}", f);

    let c_from = 'a';
    let c_to = 'z';
    let c = get_rand_from_range(c_from, c_to);
    println!("c = {}", c);
}

/// Returns a random number from a range between min and max
pub fn get_rand_from_range<T>(min: T, max: T) -> T
where
    T: rand::distributions::uniform::SampleUniform + PartialOrd,
{
    let range = RangeInclusive::new(min, max);
    rand::thread_rng().gen_range(range)
}

#[cfg(test)]
mod tests {
    // use super::*;

    /// Test `test_stub` always passes.
    #[test]
    fn test_stub() {
        assert!(true);
    }
}
