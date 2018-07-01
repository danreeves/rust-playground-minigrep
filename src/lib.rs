use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    filepath: String,
    search: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let filepath = args[1].clone();
        let search = args[2].clone();

        Ok(Config { filepath, search })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    println!("Searching '{}' for '{}'", config.filepath, config.search);

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
        let search = "bbb";
        let contents = "\
            aaa
            bbb
            ccc
            ";
        assert_eq!(vec!["bbb"], occurences(&search, &contents));
    }

}
