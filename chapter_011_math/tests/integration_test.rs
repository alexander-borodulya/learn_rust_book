use chapter_011_math::*;

mod common;

#[test]
fn integration_test_add_250_sub_750() {
    let i = 1000;

    let p1 = add(i, 250);
    assert_eq!(p1, 1250);

    let p2 = sub(i, 750);
    assert_eq!(p2, 250);
}

#[test]
#[ignore]
fn integration_test_001() {
    assert_eq!(100, 100);
}

#[test]
#[ignore]
fn integration_test_002() {
    assert_ne!(100, 200);
}

#[test]
#[ignore]
fn integration_test_003() {
    assert_eq!("get_info_string", common::get_info_string());
}
