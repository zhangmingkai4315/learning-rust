#![feature(lookup_host)]

use std::env;
use std::net::{IpAddr,SocketAddr};
use std::net::lookup_host;

fn main() {
    let local: IpAddr="127.0.0.1".parse().unwrap();
    println!("{}", local);

    let args: Vec<_> = env::args().collect();
    if args.len() != 2{
        eprintln!("please provide only one host");
        std::process.exit(1);
    }else{
        let addresses = lookup_host(&args[1]).unwrap();
        for address in addresses{
            println!("{}",address.ip())
        }
    }

}