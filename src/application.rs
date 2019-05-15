use crate::{Executor, Model};
use std::sync::{Arc, Mutex};

/// Application
/// TODO: we use N here because the notification
///       function can not be boxed because we need to clone it.
pub struct Application<M, E>
where
    M: Model<E>,
    E: 'static + Send,
{
    model: M,
    executor: Box<Executor>,
    notifier: Box<Notifier>,
    pending: Arc<Mutex<Vec<E>>>,
}

impl<M, E> Application<M, E>
where
    M: Model<E>,
    E: 'static + Send,
{
    /// Creates an application from a state that implements model, an executor,
    /// and a asynchronous notifier the informs when update should be called.
    pub fn new(
        model: M,
        executor: impl Executor + 'static,
        notifier: impl Fn() -> () + Send + 'static + Clone,
    ) -> Application<M, E> {
        Application {
            model,
            executor: Box::new(executor),
            notifier: Box::new(NotifierHandle(notifier)),
            pending: Arc::default(),
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
        self.notifier.clone_boxed()();
        self
    }

    /// Update the application's state. This delivers pending events and
    /// schedules the commands to the executor.
    pub fn update(&mut self) -> &mut Self {
        for e in self.pending.lock().unwrap().drain(..) {
            let cmd = self.model.update(e);
            for f in cmd.unpack() {
                let notify = self.notifier.clone_boxed();
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

    /// The current model state of the application.
    pub fn model(&self) -> &M {
        &self.model
    }
}

//
// Support to hide a clonable function behind a trait object.
// TODO: can this simplified?
//

struct NotifierHandle<F>(F);

trait Notifier {
    fn clone_boxed(&self) -> Box<Fn() + Send + 'static>;
}

impl<F> Notifier for NotifierHandle<F>
where
    F: Fn() + Send + 'static + Clone,
{
    fn clone_boxed(&self) -> Box<Fn() + Send + 'static> {
        Box::new(self.0.clone())
    }
}
