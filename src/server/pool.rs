use std::sync::{mpsc, Arc, Mutex};
use std::thread::{spawn, JoinHandle};

use super::request::Request;
use super::response::Response;

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Pool {
    max_threads: u8,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl Worker {
    fn new(id: usize, recv: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = spawn(move || loop {
            let job = recv.lock().unwrap().recv().unwrap();
            println!("Exec on thread {}", id);
            job();
        });
        Worker { id, thread }
    }
}

impl Pool {
    pub fn new(max: u8) -> Self {
        assert!(max > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(max as usize);

        let arc_recv = Arc::new(Mutex::new(receiver));

        for id in 0..max as usize {
            let recv = Arc::clone(&arc_recv);
            workers.push(Worker::new(id, recv));
        }

        Pool {
            max_threads: max,
            workers,
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
