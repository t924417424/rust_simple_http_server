use std::sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
};

use super::{job::Job, worker::Worker};
#[derive(Debug)]
pub struct Pool {
    // workers是一个该线程池，任务最终会在这里执行，为什么要放在这？因为不放在这的话Worker的生命周期就不够长了
    _workers: Vec<Worker>,
    // 通过这个channel的sender来为线程池派发任务
    sender: Sender<Job>,
}

impl Pool {
    pub fn new(size: usize) -> Self {
        if size <= 0 {
            panic!("size must be greater than 0");
        }
        let (sender, receiver) = channel();
        // 因为rceiver只能有一个接受者且不能被clone，否则会产生数据竞争，故这里需要通过Arc和Mutex来保护
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        println!("Number of threads : {}", size);
        Pool {
            _workers: workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
