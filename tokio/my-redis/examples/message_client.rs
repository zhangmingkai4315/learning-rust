
use mini_redis::{client};
use tokio::sync::mpsc;
use bytes::Bytes;
use tokio::sync::oneshot;

#[derive(Debug)]
enum  Command{
    Get { key: String, resp: Responder<Option<Bytes>>},
    Set { key: String, val: Bytes, resp: Responder<()>,},
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
pub async fn main(){
    let (mut tx, mut rx) = mpsc::channel(32);
    let mut tx2 = tx.clone();
    let t1 = tokio::spawn(async move{
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_owned(),
            resp: resp_tx,
        };
        tx.send(cmd).await;
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });
    let t2 =  tokio::spawn(async move{
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".to_owned(),
            val: "world".into(),
            resp: resp_tx,
        };
        tx2.send(cmd).await;
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let manager = tokio::spawn(async move{
        let mut redis_client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(message) = rx.recv().await{
            match message {
                Command::Get{
                    key,resp
                } => {
                    let res = redis_client.get(&key).await;
                    let _ = resp.send(res);
                },
                Command::Set {
                    key, val, resp
                } => {
                    let res = redis_client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
