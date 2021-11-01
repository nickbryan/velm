#![warn(clippy::all, clippy::pedantic)]

use velm_core::Editor;
use velm_tui::map_crossterm_event_stream;

#[tokio::main]
async fn main() {
    Editor::new(map_crossterm_event_stream()).run().await;
}
