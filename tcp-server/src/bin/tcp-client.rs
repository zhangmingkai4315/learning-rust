use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("unable to connect server");

    loop{
        let mut input = String::new();
        let mut buf = Vec::new();
        io::stdin().read_line(&mut input).expect("fail to read from stdin");
        stream.write(input.as_bytes()).expect("fail to write to server");

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buf).expect("fail to read from stream");
        println!("{}", str::from_utf8(&buf).expect("fail to convert to utf8"));
    }
}