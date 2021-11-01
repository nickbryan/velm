#![warn(clippy::all, clippy::pedantic)]

mod backend;
mod communication;
mod editor;

pub use backend::{Event, EventStream, Key};
pub use editor::Editor;
