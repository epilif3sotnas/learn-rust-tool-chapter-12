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

    run(config);
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

fn run (config: HashMap<&str, String>) {
    let file_content = read_file(config.get("file").unwrap_or_else(|| {
        println!("File does not exist: {:?}", config);
        exit(1);
    }));
}

fn read_file (path: &String) -> String {
    return read_to_string(path).unwrap_or_else(|err| {
        println!("Error: {:?}", err);
        exit(1);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_good_parameters () {
        let mut expected = HashMap::new();
        expected.insert("file", String::from("file.txt"));
        expected.insert("query", String::from("query"));

        let args = vec![String::from("something"), String::from("file"), String::from("query")];

        assert_eq!(Ok(expected), get_config(args));
    }

    #[test]
    fn test_get_config_no_parameters () {
        let mut expected = "Less than 2 arguments, command structure -> cargo run {file_name} {text_to_search}";

        let args = Vec::new();

        assert_eq!(Err(expected), get_config(args));
    }
}