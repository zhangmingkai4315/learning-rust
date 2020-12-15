use std::time::Instant;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Duration;

struct Delay{
    when: Instant,
}

impl Future for Delay{
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str>{
        if Instant::now() > self.when{
            println!("hello");
            Poll::Ready("done")
        }else{
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}


use tokio::sync::Notify;
use std::sync::Arc;
use std::thread;

async fn delay_v2(dur: Duration) {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify2.notify_one();
    });


    notify.notified().await;
}

// 等价于状态机管理，保存状态
enum MainFuture {
    // Initialized, never polled
    State0,
    // Waiting on `Delay`, i.e. the `future.await` line.
    State1(Delay),
    // The future has completed.
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<()>
    {
        use MainFuture::*;

        loop {
            match *self {
                State0 => {
                    /// 第一次触发Poll进入初始化状态
                    let when = Instant::now() +
                        Duration::from_millis(10);
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                    /// 传递context。 用于管理下一次poll状态
                    /// 如果服务结束直接返回Ready,否则返回Pending
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = Terminated;
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}


#[tokio::main]
async fn main(){
    let when = Instant::now() + Duration::from_millis(1000);
    let future = Delay{ when };
    let out = future.await;
    assert_eq!(out, "done");
}




