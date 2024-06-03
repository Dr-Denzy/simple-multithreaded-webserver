use std::sync::{Arc, mpsc, Mutex};
use std::thread;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("worker {id} is disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}


// type alias
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A pool of threads to execute jobs concurrently.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    eprintln!("Failed to join worker thread: {:?}", e);
                }
            }
        }
    }
}


impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    /// Sends a job to the thread pool for execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use server::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    ///
    /// pool.execute(|| {
    ///     println!("Executing a job");
    /// });
    /// ```
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Err(e) = self.sender.as_ref().unwrap().send(job) {
            eprintln!("Failed to send job to the worker: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_thread_pool_creation_with_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_thread_pool_execute() {
        let pool = ThreadPool::new(4);
        let (tx, rx) = mpsc::channel();

        pool.execute(move || {
            tx.send(true).expect("Failed to send message");
        });

        assert!(rx.recv().expect("Failed to receive message"));
    }
}




