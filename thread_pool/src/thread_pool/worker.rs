use std::thread;

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}