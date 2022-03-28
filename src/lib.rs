use std::sync::{mpsc, Arc, Mutex};
use std::thread;
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl Worker {
    /// Create a new Worker.
    ///
    /// Contanins the own id and thread
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Executing job with worker - {}", id);
            job();
        });
        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of workers in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new<'a>(size: usize) -> Result<ThreadPool, &'a str> {
        if size > 0 {
            let mut workers = Vec::with_capacity(size);

            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            return Ok(ThreadPool { workers, sender });
        } else {
            return Err("Size should be unsigned");
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
