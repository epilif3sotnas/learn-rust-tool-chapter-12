// internal

use std::env::args;


fn main() {
    let args: Vec<String> = get_args();

    println!("{:?}", args);
}


fn get_args() -> Vec<String> {
    return args().collect();
}