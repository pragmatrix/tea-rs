//! A minimal set of abstractions to create application with the Elm architecture in Rust.
//!
//! Differences to the Elm architecture:
//! - States are mutable, we trust Rust.
//! - No predefined HTML view, any model may support multiple views.

mod application;
pub use application::*;

mod cmd;
pub use cmd::*;

mod model;
pub use model::*;

mod executor;
pub use executor::*;

mod view;
pub use view::*;

use std::thread;

/// A simple exector that uses std::thread::spawn.
pub struct ThreadSpawnExecutor {}

impl Executor for ThreadSpawnExecutor {
    fn spawn(&mut self, f: Box<dyn Fn() -> () + 'static + Send>) {
        let _jh = thread::spawn(move || f());
    }
}

/// Implement `View<R>` for an application if the application's model
/// implements a `View<R>`.
impl<S, E, R> View<R> for Application<S, E>
where
    S: Model<E> + View<R>,
    E: 'static + Send,
{
    fn render(&self) -> R {
        self.state().render()
    }
}
