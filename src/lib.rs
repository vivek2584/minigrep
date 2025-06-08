use std::env;
use std::error::Error;
use std::fs;

pub struct Input {
    pub pattern: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let pattern = args[1].clone();
        let file_name = args[2].clone();
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
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }
    results
}

pub fn case_insensitive_search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
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
