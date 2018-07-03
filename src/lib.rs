use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    filepath: String,
    query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { filepath, query })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Searching '{}' for '{}'", config.filepath, config.query);

    let mut file = File::open(&config.filepath)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    println!("{}", file_contents);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "bbb";
        let contents = "\
            aaa
            bbb
            ccc
            ";
        assert_eq!(vec!["bbb"], search(&query, &contents));
    }

}
