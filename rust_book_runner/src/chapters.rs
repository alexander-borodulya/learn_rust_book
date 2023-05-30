use std::collections::HashMap;

use crate::chapter_011;
use crate::chapter_012 as chapter_012_runner;
use crate::chapter_013;
use crate::chapter_014;
use chapter_007;
use rust_book::chapter_001;
use rust_book::chapter_002;
use rust_book::chapter_003;
use rust_book::chapter_004;
use rust_book::chapter_005;
use rust_book::chapter_006;
use rust_book::chapter_008;
use rust_book::chapter_009;
use rust_book::chapter_010;
use rust_book::chapter_012 as chapter_012_samples;
use rust_book::chapter_015;
use rust_book::chapter_016;
use rust_book::chapter_017;
use rust_book::chapter_018;
use rust_book::chapter_019;

type ChaptersHashMap = HashMap<u32, Vec<fn(u32)>>;

/// Constructs HashMap with all the chapters.
pub fn make_chapters() -> ChaptersHashMap {
    let mut chapters: ChaptersHashMap = HashMap::new();
    chapters.insert(1, vec![chapter_001::run]);
    chapters.insert(2, vec![chapter_002::run]);
    chapters.insert(3, vec![chapter_003::run]);
    chapters.insert(4, vec![chapter_004::run]);
    chapters.insert(5, vec![chapter_005::run]);
    chapters
        .entry(6)
        .or_insert_with(Vec::new)
        .push(chapter_006::run_external);
    chapters
        .entry(6)
        .or_insert_with(Vec::new)
        .push(chapter_006::run);
    chapters.insert(7, vec![chapter_007::run]);
    chapters.insert(8, vec![chapter_008::run]);
    chapters.insert(9, vec![chapter_009::run]);
    chapters.insert(10, vec![chapter_010::run]);
    chapters.insert(11, vec![chapter_011::run]);
    chapters
        .entry(12)
        .or_insert_with(Vec::new)
        .push(chapter_012_samples::run);
    chapters
        .entry(12)
        .or_insert_with(Vec::new)
        .push(chapter_012_runner::run);
    chapters.insert(13, vec![chapter_013::run]);
    chapters.insert(14, vec![chapter_014::run]);
    chapters.insert(15, vec![chapter_015::run]);
    chapters.insert(16, vec![chapter_016::run]);
    chapters.insert(17, vec![chapter_017::run]);
    chapters.insert(18, vec![chapter_018::run]);
    chapters.insert(19, vec![chapter_019::run]);
    chapters
}
