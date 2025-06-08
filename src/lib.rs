use std::error::Error;
use std::fs;

pub struct Input {
    pub pattern: String,
    pub file_name: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let pattern = args[1].clone();
        let file_name = args[2].clone();

        Ok(Input {
            pattern: pattern,
            file_name: file_name,
        })
    }
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.file_name)?;

    for line in search(&input.pattern, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(pattern) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let pattern = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }
}
