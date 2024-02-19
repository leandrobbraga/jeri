use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};

type Task<'task> = Box<dyn FnOnce() -> () + Send + 'task>;

pub(crate) struct ThreadPool {
    sender: Sender<Task<'static>>,
    state: Arc<ThreadPoolState>,
}

struct ThreadPoolState {
    tasks: AtomicU64,
    condvar: Condvar,
    finished_mutex: Mutex<bool>,
}

impl Default for ThreadPool {
    fn default() -> Self {
        ThreadPool::new(std::thread::available_parallelism().unwrap().get())
    }
}

impl ThreadPool {
    pub(crate) fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let state = Arc::new(ThreadPoolState {
            tasks: AtomicU64::new(1),
            condvar: Condvar::new(),
            finished_mutex: Mutex::new(false),
        });

        for _ in 0..size {
            ThreadPool::spawn_worker(receiver.clone(), state.clone());
        }

        ThreadPool { sender, state }
    }

    pub(crate) fn with_scope<'env, 'pool: 'env, F>(&'pool self, f: F)
    where
        F: for<'scope> FnOnce(&'scope Scope<'scope, 'pool, 'env>),
    {
        {
            let scope = Scope {
                threadpool: self,
                scope: PhantomData,
                env: PhantomData,
            };
            f(&scope);
        }

        let mut finished = self.state.finished_mutex.lock().unwrap();

        // Verify if all the tasks from the scope has finished
        // IMPORTANT: The task counter started with '1' to avoid a race condition where the tasks
        //            reach '0' before all tasks were spawned. By considering the scope building
        //            as a task itself we ensure that all tasks were spawned and executed before
        //            dropping the scope.
        if self.state.tasks.fetch_sub(1, Ordering::SeqCst) > 1 {
            while !(*finished) {
                finished = self.state.condvar.wait(finished).unwrap();
            }
        }

        // Setup for the next scope
        self.state.tasks.store(1, Ordering::Release);
        *finished = false;
    }

    fn spawn_worker(receiver: Arc<Mutex<Receiver<Task<'static>>>>, state: Arc<ThreadPoolState>) {
        // FIXME: Deal with panics in the worker threads
        std::thread::spawn(move || loop {
            let execute_task = {
                let rcvr = receiver.lock().unwrap();

                let Ok(execute_task) = rcvr.recv() else { break };

                execute_task
            };

            execute_task();

            // Signal back if the last task was processed
            if state.tasks.fetch_sub(1, Ordering::SeqCst) == 1 {
                let mut finished = state.finished_mutex.lock().unwrap();
                *finished = true;
                state.condvar.notify_one();
            }
        });
    }
}

pub(crate) struct Scope<'scope, 'pool: 'env, 'env: 'scope> {
    threadpool: &'pool ThreadPool,
    scope: PhantomData<&'scope mut &'scope ()>,
    env: PhantomData<&'env mut &'env ()>,
}

impl<'scope, 'env> Scope<'scope, '_, 'env> {
    pub(crate) fn enqueue_task<F>(&'scope self, f: F)
    where
        F: FnOnce() + Send + 'scope,
    {
        // SAFETY: We ensure that all the threads finished executing 'f' before dropping the scope
        //         effectively giving `'scope` lifetime to 'f'.
        let f = unsafe { std::mem::transmute::<Task<'scope>, Task<'static>>(Box::new(f)) };

        self.threadpool.state.tasks.fetch_add(1, Ordering::SeqCst);
        self.threadpool.sender.send(f).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_vector() {
        let tp = ThreadPool::default();

        let mut my_vec = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        tp.with_scope(|scope| {
            for value in &mut my_vec {
                scope.enqueue_task(|| *value += 10);
            }
        });

        assert_eq!(my_vec, vec![10, 10, 10, 10, 10, 10, 10, 10, 10]);
    }
}
