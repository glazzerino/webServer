use std::thread;
use std::sync:: {
    mpsc,
    Arc,
    Mutex,
};
pub struct Threadpool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
trait FnBox{
    fn call_box(self:Box<Self>);
}
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
type Job = Box<dyn FnBox + Send + 'static>;

impl Threadpool {
    
    pub fn new(size : usize) -> Threadpool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Threadpool{
            workers,
            sender, 
        }
    }
    pub fn execute<F>(&self,f: F) 
        where
            F: FnOnce() + Send + 'static 
            {
                let job = Box::new(f);
                self.sender.send(job).unwrap();
            }
    
}
struct Worker {
    id: usize,
    thread_handle: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move ||{
            loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing", id);
            job.call_box();
            }
        });
        Worker {
           id: id,
           thread_handle: thread,
        }
    }
}