use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};

use std::future::Future;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc,Mutex};
use std::task::{Context, Poll};
use std::time::Duration;

use super::timer_example::TimeFuture;

struct Task{
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>
}

struct Executor{
    read_queue: Receiver<Arc<Task>>
}

#[derive(Clone)]
struct Spawner{
    task_sender: SyncSender<Arc<Task>>
}

fn new_excutor_and_spawner() -> (Executor, Spawner){
    const max_queue_size: usize = 10_000;
    let (task_sender, read_queue) = sync_channel(max_queue_size);
    (Executor{read_queue}, Spawner{task_sender})
}

impl Spawner{
    fn spawn(&self, future: impl Future<Output=()> + 'static + Send ){
        let future = future.boxed();
        let task = Arc::new(Task{
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many task queued");
    }
}

// 任务现在具有唤醒功能
impl ArcWake for Task{
    // 当执行wake的时候调用该函数
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let clone = arc_self.clone();
        arc_self.task_sender.send(clone).expect("too many tasks queue");
    }
}

impl Executor{
    fn run(&self){
        while let Ok(task) = self.read_queue.recv(){
            // 从队列中读取
            let mut future_slot = task.future.lock().unwrap();
            // 如果队列中存在数据
            if let Some(mut future) = future_slot.take(){
                // 调用waker_ref获取waker对象
                let wake = waker_ref(&task);
                // 创建Context
                let context = &mut Context::from_waker(&*wake);
                // 执行future的poll方法并将回调放入参数中
                if let Poll::Pending = future.as_mut().poll(context){
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[test]
fn test_excutor(){
    let (excutor, spawner) = new_excutor_and_spawner();
    spawner.spawn(async {
        println!("start timing...");
        TimeFuture::new(1, Duration::new(2, 0)).await;
        println!("end timing...");
    });
    // 删除后不再执行接收任何的新的任务更新
    drop(spawner);
    excutor.run();
}