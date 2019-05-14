use crate::Cmd;

/// A component, based on a State and an Event.
/// A component can be attached to any type that can be cloned.
/// TODO: probably a component sounds like too much in this context,
///       basically we attach an event to a piece of mutable state.
pub trait Component<S, E: Send> {
    /// The update function updates the current state of the component.
    /// It applies the event to it and returns the new state and,
    /// optionally a command to be executed asynchronously that sends back
    /// an event when finished.
    fn update(&mut self, event: E) -> Cmd<E>;
}
