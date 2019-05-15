use crate::{Cmd, Executor, Mailbox, Model};

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
    mailbox: Mailbox<E>,
}

impl<M, E> Application<M, E>
where
    M: Model<E>,
    E: 'static + Send,
{
    /// Creates an application from a state that implements model, an executor,
    /// and a asynchronous notifier the informs when update should be called.
    pub fn new(model: M, executor: impl Executor + 'static) -> Application<M, E> {
        Application {
            model,
            executor: Box::new(executor),
            mailbox: Mailbox::new(),
        }
    }

    /// Returns a mailbox that can be used to post events to.
    ///
    /// The mailbox returned can cloned and send to other threads.
    pub fn mailbox(&self) -> Mailbox<E> {
        self.mailbox.clone()
    }

    /// Update the application's state.
    ///
    /// This function waits for events from the mailbox, delivers them to the
    /// model and schedules the commands to the executor.
    pub fn update(&mut self) -> &mut Self {
        let events = self.mailbox.take_all();
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
            let mailbox = self.mailbox.clone();
            let async_fn = move || {
                let r = f();
                mailbox.post(r);
            };
            self.executor.spawn(Box::new(async_fn));
        }
        self
    }

    /// The current model state of the application.
    pub fn model(&self) -> &M {
        &self.model
    }
}
