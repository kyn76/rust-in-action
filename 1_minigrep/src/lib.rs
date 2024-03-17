use std::error::Error;
use std::fs;
use std::path::PathBuf;
use clap::Parser;
use regex::Regex;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Arguments {
    /// the pattern to look for
    pub query: String,

    /// the path to the file to read
    pub file_path: PathBuf,

    /// ignore case option when searching
    #[arg(short, long)]
    pub ignore_case: bool,

    /// search query as regular expression
    #[arg(short = 'e', long)]
    pub extended_regex: bool,
}


pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    let results = if args.extended_regex {
        search_regex(&args.query, &contents)
    } else if args.ignore_case {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_regex<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let mut results = Vec::new();
    let re = Regex::new(query).unwrap();
    // return all lines in which an expression matches the regex
    for line in contents.lines() {
        if re.is_match(line) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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

    #[test]
    fn case_regex() {
        let query = "[rR]ust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_regex(query, contents)
        );
    }
}