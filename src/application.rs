use crate::{Cmd, Executor, Model};
use std::mem;
use std::sync::{Arc, Mutex};

/// Application.
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
    mailbox: Arc<Mutex<Vec<E>>>,
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
        notifier: impl Fn() + Send + 'static + Clone,
    ) -> Application<M, E> {
        Application {
            model,
            executor: Box::new(executor),
            notifier: Box::new(NotifierHandle(notifier)),
            mailbox: Arc::default(),
        }
    }

    /// Post an event to the event queue.
    ///
    /// This function does not call update(),
    /// and neither invokes the notification callback,
    /// the client has to take care of that.
    pub fn post(&mut self, event: E) -> &mut Self {
        self.mailbox.lock().unwrap().push(event);
        self
    }

    /// Update the application's state.
    ///
    /// This function delivers pending events and
    /// schedules the commands to the executor.
    pub fn update(&mut self) -> &mut Self {
        let events = mem::replace(&mut *self.mailbox.lock().unwrap(), Vec::new());
        for e in events {
            let cmd = self.model.update(e);
            self.schedule(cmd);
        }
        self
    }

    /// Schedule a command to the executor.
    ///
    /// This function can be used to initiate an initial
    /// asynchronous command, or to schedule some commands externally to
    /// avoid introducing new application events.
    ///
    /// This function's self reference is mutable, because it needs the
    /// executor that runs the command to be mutable.
    // TODO: can we remove the mutability here and from the executor?
    pub fn schedule(&mut self, cmd: Cmd<E>) -> &mut Self {
        for f in cmd.unpack() {
            let notify = self.notifier.clone_boxed();
            let pending = self.mailbox.clone();
            let async_fn = move || {
                let r = f();
                pending.lock().unwrap().push(r);
                notify();
            };
            self.executor.spawn(Box::new(async_fn));
        }
        self
    }

    /// Notify the external callback that one or more new
    /// events have been posted to the queue.
    ///
    /// This directly calls the notification callback
    /// and does nothing else, expecting that the client to call update() in
    /// turn.
    pub fn notify(&self) -> &Self {
        self.notifier.clone_boxed()();
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

struct NotifierHandle<F: 'static>(F);

trait Notifier {
    fn clone_boxed(&self) -> Box<Fn() + Send>;
}

impl<F> Notifier for NotifierHandle<F>
where
    F: Fn() + Send + Clone,
{
    fn clone_boxed(&self) -> Box<Fn() + Send> {
        Box::new(self.0.clone())
    }
}
