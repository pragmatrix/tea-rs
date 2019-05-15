use crate::Cmd;

/// A model.
pub trait Model<E: Send> {
    /// The update function updates the current state of the model.
    /// It applies the event to it and returns a command that is executed
    /// asynchronously and sends back an event when finished.
    fn update(&mut self, event: E) -> Cmd<E>;
}
