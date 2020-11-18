use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0;512];
    loop{
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}


fn main() {
    let ipaddr = "0.0.0.0:8080";
    let listen = TcpListener::bind(ipaddr).expect(format!("unable bind {}", ipaddr).as_str());
    println!("listen at {}", ipaddr);
    for stream in listen.incoming(){
        match stream {
            Err(e) => {eprintln!("fail: {}", e)},
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|err|{
                        eprintln!("{:?}", err)
                    });
                });
            }
        }
    }

    println!("Hello, world!");
}
