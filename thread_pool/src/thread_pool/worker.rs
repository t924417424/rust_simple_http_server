use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use super::job::Job;

#[derive(Debug)]
pub struct Worker {
    _id: usize,
    // 这是当前worker对应的线程，为什么要放在这？因为不放在这的话thread的生命周期就不够长了
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                // 因为receiver会阻塞接受任务，且使用Arc和Mutex来保护，故这里可以使用unwrap来处理，因为并没有返回Err的时机
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            }
        });
        Worker {
            _id: id,
            _thread: thread,
        }
    }
}
