use std::thread;
use std::sync::{mpsc, Arc, Mutex};

enum Message{
    NewJob(Job),
    Terminator,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct  ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


impl Drop for ThreadPool{
    fn drop(&mut self){
        for _ in self.workers{
            self.sender.send(Message::Terminator).unwrap();
        }

        println!("shut down workers");
        for worker in &mut self.workers{
            println!("shut down workers");
            if let Some(thread) = worker.thread.take(){
               thread.join().unwrap();
            }
        }
    }
}

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{
        let thread = thread::spawn(move||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("worker {} got a job", id);
                        job();
                    },
                    Message::Terminator =>{
                        println!("receive a terminator signal");
                        break
                    }
                }

            }
        });
        Worker{
            id,
            thread: Some(thread),
        }
    }
}



impl ThreadPool{
    /// 创建线程池
    ///
    /// 线程池中的线程数量
    ///
    /// # Panics
    ///
    /// `new`函数在Size为0的时候触发Panic
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver  = Arc::new(Mutex::new(receiver));
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool{
            workers,
            sender,
        }
    }
    // 其中FnOnce()代表传递的闭包无参无返回
    // 闭包可以为Fn FnOnce或者FnMut
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static{
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

