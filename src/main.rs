use std::env;
use std::fs;
use std::process;

use lwgrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

use std::error::Error;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case;

        if args.len() < 4 {
            ignore_case = env::var("IGNORE_CASE").is_ok_and(|x| x == "1");
        } else {
            ignore_case = args[3] == "ignore_case"
        }

        Ok(Config {
            query,
            filepath,
            ignore_case,
        })
    }
}
