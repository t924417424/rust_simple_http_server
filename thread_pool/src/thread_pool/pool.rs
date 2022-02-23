use std::sync::mpsc;

use super::{job::Job, worker::Worker};
#[derive(Debug)]
pub struct Pool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
