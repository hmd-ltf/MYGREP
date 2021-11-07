use std::env;
use std::process;
use mygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem with parsing the arguments {}" , err);
        process::exit(1);
    });

    println!("Searching for {} in {}" ,config.query , config.filename);
    
    if let Err(e) = mygrep::run(config) {
        println!("Application Error {}" , e);
        process::exit(1);
    }
}
