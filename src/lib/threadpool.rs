use std::{
    error::Error,
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Clone, Debug)]
pub struct PoolCreationError {
    pub description: String,
}

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new Worker for Threadpool.
    ///
    /// The id is the identifier of the worker
    ///
    /// The receiver is the shared reference of a receiver object which will fetch job from queue in a channel
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread_result = thread::Builder::new()
            .name(format!("thread-{id}").to_string())
            .spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing...");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down...");
                        break;
                    }
                }
            });

        let thread = match thread_result {
            Ok(t) => t,
            Err(_) => panic!("Problem while creating thread for worker with id {id}"),
        };
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError {
                description: String::from("Pool creation error, size can't be zero"),
            });
        };

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// Receive the task and send them to the workers to execute
    ///
    /// f is a closure that needs to be executed
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap_or_else(|err| println!("{}", err))
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
