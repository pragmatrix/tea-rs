use crate::{Component, Executor};
use std::sync::{Arc, Mutex};

/// Application
/// TODO: we use N here because the notification
///       function can not be boxed, because we need to clone it.
pub struct Application<S, E, N>
where
    S: Component<E>,
    E: Send,
{
    state: S,
    executor: Box<Executor>,
    notify: N,
    pending: Arc<Mutex<Vec<E>>>,
}

impl<S, E, N> Application<S, E, N>
where
    S: Component<E>,
    E: 'static + Send,
    N: Fn() -> () + 'static + Send + Clone,
{
    /// Creates an application from a state that implements Component, an executor,
    /// and a asynchronous callback the informs when update should be called.
    pub fn new(state: S, executor: impl Executor + 'static, notify: N) -> Application<S, E, N> {
        Application {
            state,
            executor: Box::new(executor),
            notify,
            pending: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Schedule an event. Note that this does not call update(),
    /// and neither invokes the notification callback,
    /// the client has to take care of that.
    pub fn schedule(&mut self, event: E) -> &mut Self {
        self.pending.lock().unwrap().push(event);
        self
    }

    /// Notify the external callback that one or more new
    /// events are pending. This directly calls the notification callback
    /// and does nothing else, expecting that the client to call update() in
    /// turn.
    pub fn notify(&self) -> &Self {
        (self.notify)();
        self
    }

    /// Update the application's state. This delivers _all_ pending events and
    /// schedules the commands to the executor.
    pub fn update(&mut self) -> &mut Self {
        for e in self.pending.lock().unwrap().drain(..) {
            let cmd = self.state.update(e);
            for f in cmd.unpack() {
                let notify = self.notify.clone();
                let pending = self.pending.clone();
                let async_fn = move || {
                    let r = f();
                    pending.lock().unwrap().push(r);
                    notify();
                };
                self.executor.spawn(Box::new(async_fn));
            }
        }
        self
    }

    /// The current state of the application.
    pub fn state(&self) -> &S {
        &self.state
    }
}
