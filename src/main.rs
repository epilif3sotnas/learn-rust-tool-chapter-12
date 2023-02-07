// internal

use std::env::args;
use std::fs::read_to_string;
use std::collections::HashMap;


fn main () {
    let args: Vec<String> = get_args();
    println!("\nArgs: {:?}", args);

    let config: HashMap<&str, String> = get_config(args);
    println!("\nConfig content: {:?}", config);
}

fn get_args () -> Vec<String> {
    return args().collect();
}

fn get_config (args: Vec<String>) -> HashMap<&'static str, String> {
    let mut config = HashMap::new();
    
    if args.len() <= 2 {
        config.insert("file", String::new());
        config.insert("query", String::new());
        return config;
    }

    config.insert("file", args[1].clone() + ".txt");
    config.insert("query", args[2].clone());
    return config;
}

fn read_file (path: &String) -> String {
    return read_to_string(path).expect("Read File Error");
}