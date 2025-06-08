use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let cmdline_args: Vec<String> = env::args().collect();

    let input = Input::new(&cmdline_args).unwrap_or_else(|err| {
        println!("Failed to parse args : {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}\n", input.pattern, input.file_name);

    if let Err(e) = run(input) {
        println!("failed to read from file");
        process::exit(1);
    }
}

struct Input {
    pattern: String,
    file_name: String,
}

impl Input {
    fn new(args: &[String]) -> Result<Input, &str> {
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

fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(input.file_name)?;

    Ok(())
}
