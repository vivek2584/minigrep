use std::env;
use std::fs;

fn main() {
    let cmdline_args: Vec<String> = env::args().collect();

    let pattern = &cmdline_args[1];
    let file_name = &cmdline_args[2];

    println!("Searching for {} in {}\n", pattern, file_name);

    let contents = fs::read_to_string(file_name).expect("Unable to open specified file path!");

    println!("{}", contents);
}
