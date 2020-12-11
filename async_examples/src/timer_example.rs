use std::future::Future;
use std::sync::{Mutex,Arc};
use std::thread;
use std::time::Duration;
use std::task::{Context, Poll, Waker};
use tokio::macros::support::Pin;
use futures::join;

pub struct TimeFuture{
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState{
    completed: bool,
    waker: Option<Waker>,
    id: usize,
}

impl Future for TimeFuture{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        println!("polling: {}", shared_state.id);
        if shared_state.completed{
            Poll::Ready(())
        }else{
            // clone当前任务的的waker到共享状态中
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimeFuture{
    pub fn new(id:usize, duration: Duration)->Self{
        let shared_state =Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None,
            id,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move||{
            println!("sleep: {}", id);
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true ;
            // 如果当前的waker存在对象，则执行该对象的wake
            if let Some(waker) = shared_state.waker.take(){
                println!("try wake up : {}", id);
                // 通知excutor可以执行poll
                waker.wake();
            }
        });

        TimeFuture{
            shared_state
        }
    }
}

async fn concurrent_run(){
    let timer1 = TimeFuture::new(1, Duration::from_secs(5));
    let timer2 = TimeFuture::new(2, Duration::from_secs(5));
    let timer3 = TimeFuture::new(3, Duration::from_secs(5));
    let timer4 = TimeFuture::new(4, Duration::from_secs(5));
    let timer5 = TimeFuture::new(5, Duration::from_secs(5));
    let timer6 = TimeFuture::new(6, Duration::from_secs(5));
    join!(timer1, timer2,timer3, timer4,timer5, timer6);
}

#[test]
fn test_timer(){
    use std::time::Duration;
    use futures::executor::block_on;
    block_on(concurrent_run());
}