use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main_v1()->io::Result<()>{
    let mut socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    let (mut rd, mut wr) = io::split(socket);
    let write_task = tokio::spawn(async move{
            wr.write_all(b"hello\r\n").await?;
            wr.write_all(b"world\r\n").await?;
            Ok::<_, io::Error>(())
    });
    let mut buffer = [0;128];
    loop{
        let n: usize = rd.read(&mut buffer[..]).await?;
        if n == 0{
            break;
        }
        println!("got {:?}", &buffer[..n]);
    }
    Ok(())
}


#[tokio::main]
async fn main()->io::Result<()>{
    let mut listen = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    loop{
        let (mut socket, _) = listen.accept().await?;

        tokio::spawn(async move{
           let mut buf = vec![0;1024];
            loop{
                match socket.read(&mut buf).await {
                    Ok(0) => return ,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err(){
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}