use std::mem;
use std::sync::{Arc, Condvar, Mutex};

pub struct Mailbox<M: Send>(Arc<(Mutex<Vec<M>>, Condvar)>);

impl<M: Send> Clone for Mailbox<M> {
    fn clone(&self) -> Self {
        Mailbox(self.0.clone())
    }
}

impl<M: Send> Mailbox<M> {
    pub(crate) fn new() -> Self {
        Mailbox(Arc::default())
    }

    pub fn post(&self, e: M) {
        let &(ref lock, ref cvar) = &*self.0;
        let mut v = lock.lock().unwrap();
        v.push(e);
        cvar.notify_one();
    }

    pub(crate) fn take_all(&self) -> Vec<M> {
        let &(ref lock, ref cvar) = &*self.0;
        let mut v = lock.lock().unwrap();
        while (*v).is_empty() {
            v = cvar.wait(v).unwrap()
        }
        mem::replace(&mut *v, Vec::new())
    }
}
