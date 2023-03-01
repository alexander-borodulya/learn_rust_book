use std::collections::HashMap;

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
use crate::chapter_011;
use crate::chapter_012 as chapter_012_runner;
use rust_book::chapter_012 as chapter_012_samples;
use crate::chapter_013;
use crate::chapter_014;
use rust_book::chapter_015;
use rust_book::chapter_016;
use rust_book::chapter_017;
use rust_book::chapter_018;
use rust_book::chapter_019;

type ChaptersHashMap = HashMap<u32, fn(u32)>;

/// Constructs HashMap with all the chapters.
pub fn make_chapters() -> ChaptersHashMap {
    let mut chapters: ChaptersHashMap = HashMap::new();
    chapters.insert(1, chapter_001::run);
    chapters.insert(2, chapter_002::run);
    chapters.insert(3, chapter_003::run);
    chapters.insert(4, chapter_004::run);
    chapters.insert(5, chapter_005::run);
    chapters.insert(6, chapter_006::run);
    chapters.insert(6, chapter_006::run_external);
    chapters.insert(7, chapter_007::run);
    chapters.insert(8, chapter_008::run);
    chapters.insert(9, chapter_009::run);
    chapters.insert(10, chapter_010::run);
    chapters.insert(11, chapter_011::run);
    chapters.insert(12, chapter_012_samples::run);
    chapters.insert(12, chapter_012_runner::run);
    chapters.insert(13, chapter_013::run);
    chapters.insert(14, chapter_014::run);
    chapters.insert(15, chapter_015::run);
    chapters.insert(16, chapter_016::run);
    chapters.insert(17, chapter_017::run);
    chapters.insert(18, chapter_018::run);
    chapters.insert(19, chapter_019::run);
    chapters
}
