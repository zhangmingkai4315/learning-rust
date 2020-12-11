use std::thread;
use futures::executor::block_on;
use futures::join;
use async_std::task::sleep;

fn get_two_sites(){
    let thread_one = thread::spawn(|| println!("will down load google.com"));
    let thread_two = thread::spawn(|| println!("will down load amazon.com"));

    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

async fn do_async(){
    println!("async call");
}

async fn learn_song(){
    println!("learn song");
    // this will block so not run perfect for switch context
    // sleep(std::time::Duration::from_secs(2));
    sleep(std::time::Duration::from_secs(2)).await;
    println!("done learn song");
}
async fn sing_song(){
    println!("sing song");
}

async fn dance(){
    println!("dance");
}

async fn learn_and_sing(){
    // 顺序执行，但是不会阻塞调用线程执行，因此任何同时执行的会被先执行
    learn_song().await;
    sing_song().await;
}


#[test]
fn test_thread(){
    get_two_sites();
}

#[test]
fn test_async(){
    let async_future = do_async();
    block_on(async_future);

    block_on(learn_song());
    block_on(sing_song());
    block_on(dance());
}


async fn learn_sing_dance(){
    let f1 = learn_and_sing();
    let f2 = dance();
    // 同时执行而不是等待上一个完成
    futures::join!(f1, f2);
}
#[test]
fn test_async_await(){
    block_on(learn_sing_dance());
}


mod future_example;
mod timer_example;

