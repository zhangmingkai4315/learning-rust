use super::arguments::Argument;
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};

pub struct Scanner {
    ipaddr: IpAddr,
    threads: u16,
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect(((addr, port))) {
            Ok(_) => {
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        port += num_threads;
        if port > 65535 {
            break;
        }
    }
}

impl Scanner {
    pub fn new(arguments: &Argument) -> Scanner {
        Scanner {
            ipaddr: arguments.ipaddr,
            threads: arguments.threads,
        }
    }
    pub fn run(&self) -> Vec<u16> {
        let (tx, rx) = mpsc::channel();
        for i in 0..self.threads {
            let tx = tx.clone();
            let addr = self.ipaddr.clone();
            let threads = self.threads;
            std::thread::spawn(move || {
                scan(tx, i, addr, threads);
            });
        }
        drop(tx);
        let mut result = vec![];
        for p in rx {
            result.push(p);
        }
        result.sort();
        return result;
    }
}
