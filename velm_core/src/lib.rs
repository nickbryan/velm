#![warn(clippy::all, clippy::pedantic)]

mod backend;
mod editor;

pub use backend::{Event, EventStream, Key};
pub use editor::Editor;
