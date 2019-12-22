use crate::{Cmd, Executor, Mailbox, Model};

/// Application.
/// TODO: we use N here because the notification
///       function can not be boxed because we need to clone it.
pub struct Application<M, Msg>
where
    M: Model<Msg>,
    Msg: 'static + Send,
{
    mailbox: Mailbox<Msg>,
    model: M,
    executor: Box<dyn Executor>,
}

impl<M, Msg> Application<M, Msg>
where
    M: Model<Msg>,
    Msg: 'static + Send,
{
    /// Creates an application from a mailbox, an initial state and an executor.
    pub fn new(
        mailbox: Mailbox<Msg>,
        model: M,
        executor: impl Executor + 'static,
    ) -> Application<M, Msg> {
        Application {
            mailbox,
            model,
            executor: Box::new(executor),
        }
    }

    /// Returns a mailbox that can be used to post messages to.
    ///
    /// The mailbox returned can be cloned and send to other threads.
    pub fn mailbox(&self) -> Mailbox<Msg> {
        self.mailbox.clone()
    }

    /// Update the application's state.
    ///
    /// This function waits for messages from the mailbox, delivers them to the
    /// model and schedules the commands to the executor.
    pub fn update(&mut self) -> &mut Self {
        let messages = self.mailbox.take_all();
        for msg in messages {
            let cmd = self.model.update(msg);
            self.schedule(cmd);
        }
        self
    }

    /// Schedule a command to the executor.
    ///
    /// This function can be used to schedule asynchronous commands.
    ///
    /// This function's self reference is mutable, because it needs the
    /// executor that runs the command to be mutable.
    // TODO: can we remove the mutability here and from the executor?
    pub fn schedule(&mut self, cmd: Cmd<Msg>) -> &mut Self {
        for f in cmd.unpack() {
            let mailbox = self.mailbox.clone();
            let async_fn = move || {
                mailbox.post(f());
            };
            self.executor.spawn(Box::new(async_fn));
        }
        self
    }

    /// The current model of the application.
    pub fn model(&self) -> &M {
        &self.model
    }

    /// The current model of the application.
    pub fn model_mut(&mut self) -> &mut M {
        &mut self.model
    }
}
