use std::error::Error;
use clap::Parser;
use clap::error::ErrorKind;

/// Run specific chapter and subchapter.
/// 
/// To run a specific chapter and subchapter, you should specify
/// the chapter ID in the format X.Y, 
/// where X is the chapter index and Y is the subchapter index. 
/// 
/// For example, to run Chapter 3, Subchapter 2, you would specify 3.2:
/// 
/// $ rust_book_runner -c 3.2
/// 
/// If the chapter ID consists of a single number, that means all subchapters 
/// under the specified chapter number will be run.
/// 
/// If no chapter ID is specified, the most recent chapter will be run.
#[derive(Debug, Parser)]
pub struct Args {    
    /// Chapter ID. For example, `10.1`
    #[arg(short, long, value_parser = ChapterIdParser, default_value_t = ChapterID::new(0, 0))]
    pub chapter_id: ChapterID,
    
    /// List available chapters
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
}

/// Internal type for the Chapter ID. Anonymous struct of two `u32` that represents
/// chapter and sub-chapter respectively.
#[derive(Clone, Debug, PartialEq)]
pub struct ChapterID {
    pub chapter: u32,
    pub subchapter: u32,
}

impl ChapterID {
    pub fn new(chapter_id: u32, subchapter_id: u32) -> ChapterID {
        ChapterID {
            chapter: chapter_id, subchapter: subchapter_id,
        }
    }
}

/// `std::fmt::Display` implementation for the `ChapterID` type.
impl std::fmt::Display for ChapterID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChapterID ({}, {})", self.chapter, self.subchapter)
    }
}

/// Parse string representation of the `ChapterID` type.
/// Returns ChapterID instance.
fn parse_chapter_subchapter(s: &str) -> Result<ChapterID, Box<dyn Error + Send + Sync + 'static>> {

    // We are expect here a single number that represents the chapter number only.
    // Subchapter number is equal to 0 in this case.
    if !s.chars().any(|c| c == '.' || c == ',') {
        return Ok(ChapterID::new(s.trim().parse()?, 0));
    }

    let s = s.replace(" ", "");
    let s = s.trim_start_matches("ChapterID(");
    let s = s.trim();
    let s = s.trim_end_matches(')');
    let s = s.trim();
    
    let pos = s
        .find(|c| c == '.' || c == ',')
        .ok_or_else(|| format!("Invalid (index.sub-index) value: no `.` found in `{s:?}`"))?;

    Ok(ChapterID::new(
        s[..pos].trim().parse()?, 
        s[pos + 1..].trim().parse()?
    ))
}

/// Custom chapter ID parser, which would be used for the default value for the `chapter_id` field.
#[derive(Clone)]
struct ChapterIdParser;

impl clap::builder::TypedValueParser for ChapterIdParser {
    type Value = ChapterID;
   
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let Some(arg_value) = value.to_str() else {
            let err = clap::Error::new(ErrorKind::ValueValidation)
                .with_cmd(&cmd);
            return Err(err);
        };
        
        let chapter_id = match parse_chapter_subchapter(arg_value) {
            Ok(chapter_id) => chapter_id,
            Err(_e) => ChapterID::new(0, 0)
        };
        
        Ok(chapter_id)
   }
}

/// CLI Args parser unit-tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chapter_subchapter_0_0() {
        assert_eq!(ChapterID::new(0, 0), parse_chapter_subchapter("0.0").unwrap());
        assert_eq!(ChapterID::new(0, 0), parse_chapter_subchapter("0,0").unwrap());
        assert_eq!(ChapterID::new(0, 0), parse_chapter_subchapter("ChapterID(0. 0)").unwrap());
        assert_eq!(ChapterID::new(0, 0), parse_chapter_subchapter("ChapterID(0, 0)").unwrap());
    }

    #[test]
    fn test_parse_chapter_subchapter_2_3() {
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("2.3").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("2,3").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("ChapterID(2,3)").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("ChapterID(2, 3)").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("ChapterID(2 ,3)").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("ChapterID(2 , 3)").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("ChapterID( 2 , 3)").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("ChapterID(2 , 3 )").unwrap());
        assert_eq!(ChapterID::new(2, 3), parse_chapter_subchapter("ChapterID( 2 , 3 )").unwrap());
    }

    #[test]
    fn test_parse_chapter_subchapter_5() {
        assert_eq!(ChapterID::new(5, 0), parse_chapter_subchapter("5").unwrap());
        assert_eq!(ChapterID::new(5, 0), parse_chapter_subchapter(" 5").unwrap());
        assert_eq!(ChapterID::new(5, 0), parse_chapter_subchapter("5 ").unwrap());
        assert_eq!(ChapterID::new(5, 0), parse_chapter_subchapter(" 5 ").unwrap());
    }
}
