use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use futures::task;
use crossbeam::channel;
use std::sync::{Arc, Mutex};
use futures::task::ArcWake;


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
            let waker = cx.waker().clone();
            let when = self.when;

            std::thread::spawn(move|| {
                let now = Instant::now();
                if now < when{
                    std::thread::sleep(when - now);
                }
                waker.wake();
            });

            Poll::Pending
        }
    }
}




struct Task{
    future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>,
    executor : channel::Sender<Arc<Task>>,
}

impl Task{
    fn sechedule(self: &Arc<Self>){
        self.excutor.send(self.clone());
    }

    /// 1. 创建waker
    /// 2. 根据waker创建context
    /// 3. 执行future的poll方法，并传递对应的context
    /// 将future与task进行关联在一起，当资源变成可用的时候task的waker得到通知
    /// 调度器接收到notification后 执行任务
    fn poll(self: Arc<Self>){
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }

    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where F: Future<Output=()> + Send + 'static
    {
        let task = Arc::new(Task{
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });
        let _ = sender.send(task);
    }
}

impl ArcWake for Task{
    fn wake_by_ref(arc_self: &Arc<Self>){
        arc_self.sechedule();
    }
}

struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio{
    fn new() -> MiniTokio{
        let (sender, scheduled) = channel::unbounded();
        MiniTokio{
            scheduled, sender
        }
    }

    fn spawn<F>(&mut self, future: F)
    where F: Future<Output = ()> + Send + 'static
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&mut self){
        /// 只要队列中有任何数据，则执行任务本身的poll
       while let Ok(task) = self.scheduled.recv(){
           task.poll();
       }
    }
}

// fn main(){
//     let mut mini_tokio =
// }