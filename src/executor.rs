pub trait Executor: Send {
    /// Spawn a function that needs to be run asynchronously.
    // Boxing is done here because otherwise spawn would be generic and so
    // we could not box the Executor itself, which Application does.
    fn spawn(&mut self, f: Box<dyn FnOnce() + Send>);
}
