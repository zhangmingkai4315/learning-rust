use futures::Future;
use futures::executor::block_on;

async fn foo() -> u8 {
    43
}

fn bar()->impl Future<Output=u8>{
    async{
        // 如果此时程序被阻塞则会出让执行权
        let x: u8 = foo().await;
        // 当程序可以继续执行的时候，调度器会重新恢复执行该位置后的代码
        x+1
    }
}

async fn blocks(){
    let s = "hello1".to_string();
    let future_one = async {
        println!("{}", s);
    };
    let future_two = async {
        println!("{}", s);
    };
    futures::join!(future_one, future_two);
}

fn move_block()->impl Future<Output=()>{
    let s = "hello2".to_string();
    async move {
        println!("{}", s);
    }


}

#[test]
fn test(){
    block_on(blocks());
    block_on(move_block());
}
