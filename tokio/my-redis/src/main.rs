use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bytes::Bytes;
use std::ptr::hash;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    let shardDB = Arc::new(Vec::new());
    loop {
        let (socket, _ ) = listener.accept().await.unwrap();
        // let db = db.clone();
        let db = shardDB.clone();
        tokio::spawn(async move {
            // process_v3(socket, db).await;
            process_v4(socket, db);
        });
    }
}



async fn process(socket: TcpStream){
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap(){
        println!("got: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}

async fn process_v1(socket: TcpStream){
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    let mut connection = Connection::new(socket);
    let mut db = HashMap::new();
    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                if let Some(v) = db.get(cmd.key()){
                    Frame::Bulk(v.clone().into())
                }else{
                    Frame::Null
                }
            }
            cmd=> {
                unimplemented!()
            }
        };
        connection.write_frame(&response).await.unwrap();
    }
}


async fn process_v3(socket: TcpStream, db: Db){
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let mut db = db.lock().unwrap();
                if let Some(v) = db.get(cmd.key()){
                    Frame::Bulk(v.clone().into())
                }else{
                    Frame::Null
                }
            }
            cmd=> {
                unimplemented!()
            }
        };
        connection.write_frame(&response).await.unwrap();
    }
}


async fn process_v4(socket: TcpStream, db: ShardDb){
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let shard = db[hash(cmd.key()) % db.len()].lock().unwrap();
                if let Some(v) = shard.get(cmd.key()){
                    Frame::Bulk(v.clone().into())
                }else{
                    Frame::Null
                }
            }
            cmd=> {
                unimplemented!()
            }
        };
        connection.write_frame(&response).await.unwrap();
    }
}