
use crate::chapters::make_chapters;
use crate::cli_args::Args;

pub struct Runner;

impl Runner {
    pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
        let chapters = make_chapters();

        if args.list {
            let mut l: std::vec::Vec<u32> = chapters.keys().cloned().collect();
            l.sort();
            for i in l {
                println!("{}", i);
            }
            return Ok(());
        }

        let fn_index = match args.chapter_id.chapter {
            0 => chapters.len() as u32,
            _ => args.chapter_id.chapter
        };

        if let Some(ch_fn) = chapters.get(&fn_index) {
            ch_fn(args.chapter_id.subchapter);
        }

        Ok(())
    }
}
