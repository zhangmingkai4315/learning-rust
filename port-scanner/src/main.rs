use slib::{arguments::Argument, scanner::Scanner};
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
fn main() {
    let arg: Vec<String> = env::args().collect();
    match Argument::new(&arg) {
        Ok(argument) => {
            let scanner = Scanner::new(&argument);
            let result = scanner.run();
            for i in result {
                println!("open {}", i);
            }
        }
        Err(err) => {
            println!("Err: {}", err);
            std::process::exit(1);
        }
    }
}
