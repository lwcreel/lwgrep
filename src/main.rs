use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

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
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}
