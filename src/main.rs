use minigrep::Input;
use std::env;
use std::process;

fn main() {
    let input = Input::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse args : {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}\n", input.pattern, input.file_name);

    if let Err(e) = minigrep::run(input) {
        eprintln!("failed to read from file");
        process::exit(1);
    }
}
