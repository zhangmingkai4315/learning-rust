use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;
use helloweb::ThreadPool;
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (filename, status) = if buffer.starts_with(get){
        ("hello.html", 200)
    }else if buffer.starts_with(sleep){
        // 整个服务器由于运行在单线程上面，因此后续都会被该处理拖慢
        thread::sleep(Duration::from_secs(5));
        ("404.html", 404)
    }else{
        ("404.html", 404)
    };
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {} OK\r\n\r\n {}",status, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7898").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }
}
