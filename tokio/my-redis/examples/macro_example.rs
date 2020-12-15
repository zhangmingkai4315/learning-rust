use tokio::sync::oneshot;

#[tokio::main]
async fn main_v1(){
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    // 对于未接收到的信息，直接丢弃
    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn( async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        var = rx1 => {
            println!("receive from channel one")
        }

        var = rx2 => {
            println!("receive from channel two")
        }
    }
}

#[tokio::main]
async fn main(){
  let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async{
       tokio::select! {
           val = some_operation() => {
               let _ = tx1.send(val);
           }
           _ = tx1.closed() => {

           }
       }
    });

    let out = tokio::select! {
        res1 = compution1() => res1,
        res2 = compution2() => res2,
    };

    println!("got = {}", out)
}

