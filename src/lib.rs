use std::env;
use std::error::Error;
use std::fs;

pub struct Input {
    pub pattern: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Input {
    pub fn new(mut args: env::Args) -> Result<Input, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Pattern not present"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("File name not present"),
        };

        let case_sensitive = !(env::var("CASE_SENSITIVE").is_err());

        Ok(Input {
            pattern: pattern,
            file_name: file_name,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.file_name)?;

    let results = if input.case_sensitive {
        case_sensitive_search(&input.pattern, &contents)
    } else {
        case_insensitive_search(&input.pattern, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn case_sensitive_search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn case_insensitive_search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            case_sensitive_search(pattern, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUst";
        let contents = "Rust:\nsafe, fast, productive.\nTrust?";

        assert_eq!(
            vec!["Rust:", "Trust?"],
            case_insensitive_search(pattern, contents)
        );
    }
}
