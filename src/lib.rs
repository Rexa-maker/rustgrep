use regex::Regex;
use std::{env, error::Error, fs};

static PROGRAM_NAME: &str = "rustgrep";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new(query).unwrap();
    contents.lines().filter(|line| re.is_match(line)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new(format!("(?i){query}").as_str()).unwrap();
    contents.lines().filter(|line| re.is_match(line)).collect()
}

pub fn usage() {
    eprintln!(
        "\
Help:
Will search a regular expression in a file, printing the matching lines in the standard output.

usage:
{PROGRAM_NAME} query file_path
    query: a regular expression to search
    file_path: the file in which to search
"
    );
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => {
                usage();
                return Err("Didn't get a query string");
            }
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                usage();
                return Err("Didn't get a file path");
            }
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
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
        assert_eq!(vec!["safe, fast, productive."], search(&query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&query, contents)
        );
    }
}
