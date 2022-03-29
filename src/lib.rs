use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
struct Worker {
    id: usize,
    work_sender: mpsc::Sender<Message>,
    work_thread: Option<thread::JoinHandle<()>>,
    pools: Vec<Pool>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Pool {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Pool {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Pool {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Pool {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Pool {} was told to terminate.", id);
                    break;
                }
            }
        });

        Pool {
            id,
            thread: Some(thread),
        }
    }
}
enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    /// Create a new Worker.
    ///
    /// Contanins the own id and thread
    fn new(pools_size: usize, id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let (work_sender, pool_receiver) = mpsc::channel();
        // let work_sender = Mutex::new(work_sender);
        let pool_receiver = Arc::new(Mutex::new(pool_receiver));

        let mut pools = Vec::with_capacity(pools_size);

        for i in 0..pools_size {
            pools.push(Pool::new(i, Arc::clone(&pool_receiver)));
        }
        let clone_sender = work_sender.clone();
        let work_thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; sending.", id);
                    clone_sender.send(Message::NewJob(job)).unwrap();
                }
                Message::Terminate => {
                    println!("Pool {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            work_sender,
            work_thread: Some(work_thread),
            pools,
        }
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
                workers.push(Worker::new(size, id, Arc::clone(&receiver)));
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         println!("Sending terminate message to all workers.");
//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         println!("Shutting down all workers.");
//         for worker in &mut self.workers {
//             println!("Shutting down worker {}", worker.id);

//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }
