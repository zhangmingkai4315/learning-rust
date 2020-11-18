use std::thread;
use std::net::UdpSocket;

fn main(){
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("bind socket fail");

    loop{
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("fail to clone socket");
        match sock.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn( move || {
                    sock.send_to(&buf, &src).expect("send response fail")
                });
            },
            Err(err) => {
                eprintln!("err: {}", err);
            }
        }
    }
}