//! A minimal set of abstractions to create application with the Elm architecture in Rust.
//!
//! Differences to the Elm architecture:
//! - States are mutable, we trust Rust.
//! - View type agnostic, and models may support multiple views.

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

/// A simple executor that uses std::thread::spawn.
pub struct ThreadSpawnExecutor {}

impl Default for ThreadSpawnExecutor {
    fn default() -> Self {
        ThreadSpawnExecutor {}
    }
}

impl Executor for ThreadSpawnExecutor {
    fn spawn(&mut self, f: Box<dyn Fn() + Send>) {
        let _jh = thread::spawn(move || f());
    }
}

/// Implements `View<R>` for all applications of which its model
/// implements a `View<R>`.
impl<S, E, R> View<R> for Application<S, E>
where
    S: Model<E> + View<R>,
    E: 'static + Send,
{
    fn render(&self) -> R {
        self.model().render()
    }
}
