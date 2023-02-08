// internal

use std::env::args;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::process::exit;


fn main () {
    let args: Vec<String> = get_args();
    println!("\nArgs: {:?}", args);

    let config: HashMap<&str, String> = get_config(args).unwrap_or_else(|err| {
        println!("Problem: {:?}", err);
        exit(1);
    });
    println!("\nConfig content: {:?}", config);
}

fn get_args () -> Vec<String> {
    return args().collect();
}

fn get_config (args: Vec<String>) -> Result<HashMap<&'static str, String>, &'static str> {
    if args.len() <= 2 {
        return Err("Less than 2 arguments, command structure -> cargo run {file_name} {text_to_search}");
    }

    let mut config = HashMap::new();
    config.insert("file", args[1].clone() + ".txt");
    config.insert("query", args[2].clone());
    return Ok(config);
}

fn read_file (path: &String) -> String {
    return read_to_string(path).expect("Read File Error");
}