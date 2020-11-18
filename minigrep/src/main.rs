
use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("parse config fail: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config){
        eprintln!("application error: {}", err);
        process::exit(1);
    }
}
