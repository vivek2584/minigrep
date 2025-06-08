use minigrep::Input;
use std::env;
use std::process;

fn main() {
    let cmdline_args: Vec<String> = env::args().collect();

    let input = Input::new(&cmdline_args).unwrap_or_else(|err| {
        println!("Failed to parse args : {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}\n", input.pattern, input.file_name);

    if let Err(e) = minigrep::run(input) {
        println!("failed to read from file");
        process::exit(1);
    }
}
