use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    let contents =
        fs::read_to_string(config.filepath).expect("Should have been able to read the file");

    println!("With text: \n{contents}")
}

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        Config { query, filepath }
    }
}
