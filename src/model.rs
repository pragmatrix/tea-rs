use crate::Cmd;

/// A model.
pub trait Model<Msg: Send> {
    /// The update function updates the current state of the model.
    /// It applies the msg to it and returns a command that is executed
    /// asynchronously and sends back an msg when finished.
    fn update(&mut self, msg: Msg) -> Cmd<Msg>;
}
