use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    filepath: String,
    query: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();
        let case_insensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_ok();

        Ok(Config {
            filepath,
            query,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Searching '{}' for '{}'\n", config.filepath, config.query);

    let mut file = File::open(&config.filepath)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let results = if config.case_insensitive {
        case_insensitive_search(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };

    for line in &results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'c>(query: &str, contents: &'c str) -> Vec<&'c str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn case_insensitive_search<'c>(query: &str, contents: &'c str) -> Vec<&'c str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "bbb";
        let contents = "aaa\nbbb\nccc";
        assert_eq!(vec!["bbb"], search(&query, &contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "bbb";
        let contents = "aaa\nbbb\nBBB\nccc";
        assert_eq!(
            vec!["bbb", "BBB"],
            case_insensitive_search(&query, &contents)
        );
    }

}
