// internal

use std::env::args;
use std::fs::read_to_string;


fn main() {
    let args: Vec<String> = get_args();
    println!("\nArgs: {:?}", args);

    let file = read_file(&args[1]);
    println!("\nFile content: {:?}", file);
}

fn get_args() -> Vec<String> {
    return args().collect();
}

fn read_file (path: &String) -> String {
    let file_name = path.to_owned() + ".txt";
    return read_to_string(file_name).expect("Read File Error");
}