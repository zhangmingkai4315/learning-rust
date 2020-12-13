pub async fn say_hello(){
    println!("{}","hello".to_owned());
}
use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()>{
    let op = say_hello();
    // .await如果程序阻塞则出让控制权， 由于当前仅有main函数在运行，因此程序也不会做其他的事情
    // 但是大部分的程序会包含很多任务，直接切换到其他任务中执行。
    let mut redis_client = client::connect("127.0.0.1:6379").await?;
    redis_client.set("hello", "world".into()).await?;

    let result = redis_client.get("hello").await?;
    println!("got result from redis = {:?}", result);
    // lazy run
    op.await;
    Ok(())
}
