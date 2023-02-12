use rust_book::chapter_001;
use rust_book::chapter_002;
use rust_book::chapter_003;
use rust_book::chapter_004;
use rust_book::chapter_005;
use rust_book::chapter_006;
use chapter_007;
use rust_book::chapter_008;
use rust_book::chapter_009;
use rust_book::chapter_010;
mod chapter_011;
mod chapter_012;
use crate::chapter_012 as chapter_012_runner;
use rust_book::chapter_012 as chapter_012_samples;
mod chapter_013;
mod chapter_014;
use rust_book::chapter_015;
use rust_book::chapter_016;
use rust_book::chapter_017;
use rust_book::chapter_018;
use rust_book::chapter_019;

fn main() {
    chapter_001::run();
    chapter_002::run();
    chapter_003::run();
    chapter_004::run();
    chapter_005::run();
    chapter_006::run();
    chapter_006::run_external();
    chapter_007::run();
    chapter_008::run();
    chapter_009::run();
    chapter_010::run();
    chapter_011::run();
    chapter_012_samples::run();
    chapter_012_runner::run();
    chapter_013::run();
    chapter_014::run();
    chapter_015::run();
    chapter_016::run();
    chapter_017::run();
    chapter_018::run();
    chapter_019::run();
}
