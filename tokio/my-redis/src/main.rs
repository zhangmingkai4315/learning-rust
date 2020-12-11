mod lib;

use lib::say_hello;
use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()>{
    let op = say_hello();
    let mut redis_client = client::connect("127.0.0.1:6379").await?;
    redis_client.set("hello", "world".into()).await?;

    let result = redis_client.get("hello").await?;
    println!("got result from redis = {:?}", result);
    // lazy run
    op.await;
    Ok(())
}

// #[tokio::main]
// async fn main(){
//     println!("{}", "hello");
// }
//
// fn main(){
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         println!("{}", "hello");
//     })
// }