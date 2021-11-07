use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<() , Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.is_case_sensitive {
        search(&config.query , &contents)
    } else {
        case_insensative_search(&config.query , &contents)
    };

    for line in result{
        println!("{}" , line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config , &str> {
        if args.len() < 3 {
            return Err("Not Enough Parameters");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok (Config {query , filename, is_case_sensitive})
 }
}

pub fn search<'a>(query: &str , content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
pub fn case_insensative_search<'a>(query: &str , content: &'a str)-> Vec<&'a str> {
    
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."] , search(query, content));
    }

    #[test]
    fn sensative() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
DUCT SOmeTHING.";
        assert_eq!(vec!["safe, fast, productive."] , search(query , content));
    }

    #[test]
    fn in_sensative() {
        let query = "RusT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust ME BRO";
        assert_eq!(vec!["Rust:", "Trust ME BRO"] ,case_insensative_search(query , content));
    }



}