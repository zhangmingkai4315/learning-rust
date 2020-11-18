use std::net::UdpSocket;
use std::{str, io};

fn main(){
    let socket = UdpSocket::bind("127.0.0.1:8088").expect("fail to bind client socket");
    socket.connect("127.0.0.1:8080").expect("connect to server fail");
    loop{
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        io::stdin().read_line(&mut input).expect("fail to read line ");
        socket.send(input.as_bytes()).expect("fail to write to server");

        socket.recv_from(&mut buffer).expect("read buffer error");
        println!("{}", str::from_utf8(&buffer).expect("fail to convert to utf8"));
    }
}