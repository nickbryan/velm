#![warn(clippy::all, clippy::pedantic)]

mod communication;
mod component;
mod editor;
mod input;

pub use editor::Editor;
pub use input::{Event, EventStream, Key};
