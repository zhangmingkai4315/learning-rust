use std::rc::Rc;
use tokio::task::yield_now;


async fn work(){
    {
        let rc = Rc::new(7);
        println!("{}", rc);
    }
    yield_now().await;
}

/// work! 状态得到保存？
async fn not_work(){
    let rc = Rc::new("hello");
    yield_now().await;
    println!("{}", rc);
}

#[tokio::main]
pub async fn main(){
    not_work().await;
    work().await;
}