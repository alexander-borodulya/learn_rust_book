use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        args.next(); // skip the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("No query provided!");
            }
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("No file path provided!");
            }
        };

        let ignore_case_env_var = env::var("IGNORE_CASE").is_ok();
        let ignore_case_arg_var = env::args().any(|s| s == "-i" || s == "--ignore-case" );
        let ignore_case = ignore_case_env_var || ignore_case_arg_var;
        
        Ok(Config {
            query, 
            file_path, 
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    content: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = ", two, t";
        let content = "\
one, one, one
one, two, three
1, 2, 3";
        
        assert_eq!(vec!["one, two, three"], search(query, content));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
