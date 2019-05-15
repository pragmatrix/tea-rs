use std::mem;
use std::sync::{Arc, Condvar, Mutex};

pub struct Mailbox<E: Send>(Arc<(Mutex<Vec<E>>, Condvar)>);

impl<E: Send> Clone for Mailbox<E> {
    fn clone(&self) -> Self {
        Mailbox(self.0.clone())
    }
}

impl<E: Send> Mailbox<E> {
    pub(crate) fn new() -> Self {
        Mailbox(Arc::default())
    }

    pub fn post(&self, e: E) {
        let &(ref lock, ref cvar) = &*self.0;
        let mut v = lock.lock().unwrap();
        v.push(e);
        cvar.notify_one()
    }

    pub(crate) fn take_all(&self) -> Vec<E> {
        let &(ref lock, ref cvar) = &*self.0;
        let mut v = lock.lock().unwrap();
        while (*v).is_empty() {
            v = cvar.wait(v).unwrap()
        }
        mem::replace(&mut *v, Vec::new())
    }
}
