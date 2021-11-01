#![warn(clippy::all, clippy::pedantic)]

use velm_core::Editor;
use velm_tui::map_crossterm_event_stream;

#[tokio::main]
async fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();

    Editor::default()
        .consume(map_crossterm_event_stream())
        .await;

    crossterm::terminal::disable_raw_mode().unwrap();
}
