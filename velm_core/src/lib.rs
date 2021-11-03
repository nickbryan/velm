#![warn(clippy::all, clippy::pedantic)]

mod communication;
mod component;
mod editor;
mod input;
mod render;
mod ui;

pub use editor::Editor;
pub use input::{Event, EventStream, Key};
pub use render::{Canvas, Cell};
pub use ui::{Color, Rect};
