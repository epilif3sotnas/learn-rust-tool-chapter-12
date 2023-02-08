// internal

use std::env::args;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::process::exit;


fn main () {
    let args: Vec<String> = get_args();
    println!("\nArgs: {:?}", args);

    let config: HashMap<&str, String> = get_config(args).unwrap_or_else(|err| {
        eprintln!("Problem: {:?}", err);
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
        eprintln!("File does not exist: {:?}", config);
        exit(1);
    }));

    let query = config.get("query").unwrap_or_else(|| {
        eprintln!("Query does not exist: {:?}", config);
        exit(1);
    });

    let lines_containing = search(file_content, query.clone()).unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        exit(1);
    });

    println!("Lines containing query {:?}: {:?}", config.get("query").unwrap(), lines_containing);
}

fn read_file (path: &String) -> String {
    return read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Error: {:?}", err);
        exit(1);
    });
}

fn search (text: String, query: String) -> Result<Vec<String>, &'static str> {
    if text.trim().is_empty() {
        return Err("Text with no content");
    }

    if query.trim().is_empty() {
        return Err("Query with no content");
    }

    let mut lines_with_query: Vec<String> = Vec::new();

    for line in text.lines() {
        if line.contains(&query) {
            lines_with_query.push(line.to_string());
        }
    }
    return Ok(lines_with_query);
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

    #[test]
    fn test_search_no_parameters () {
        let text = String::new();
        let query = String::new();

        let reality = search(text, query);

        let expected = Err("Text with no content");

        assert_eq!(reality, expected);
    }

    #[test]
    fn test_search_no_query () {
        let text = String::from("Text");
        let query = String::new();

        let reality = search(text, query);

        let expected = Err("Query with no content");

        assert_eq!(reality, expected);
    }

    #[test]
    fn test_search_no_text () {
        let text = String::new();
        let query = String::from("query");

        let reality = search(text, query);

        let expected = Err("Text with no content");

        assert_eq!(reality, expected);
    }

    #[test]
    fn test_search_not_contained () {
        let text = String::from("asd\nawdxa\nasxd\n");
        let query = String::from("query");

        let reality = search(text, query);

        let expected = Ok(Vec::new());

        assert_eq!(reality, expected);
    }

    #[test]
    fn test_search_contained () {
        let text = String::from("asd\nawdxa\nasxd\n");
        let query = String::from("asd");

        let reality = search(text, query);

        let expected = Ok(vec![String::from("asd")]);

        assert_eq!(reality, expected);
    }
}