use crate::Cmd;

/// A trait describing a model.
pub trait Model<Msg: Send> {
    /// The update function updates the current state of the model.
    ///
    /// It applies uses the Msg to update its state and returns a command that is executed
    /// asynchronously and is meant to send back a Msg when finished.
    fn update(&mut self, msg: Msg) -> Cmd<Msg>;
}
