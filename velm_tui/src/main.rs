#![warn(clippy::all, clippy::pedantic)]

use anyhow::{Context, Error};
use velm_core::Editor;
use velm_tui::{map_crossterm_event_stream, CrosstermCanvas};

#[tokio::main]
async fn main() -> Result<(), Error> {
    use anyhow::Context;

    let mut canvas =
        CrosstermCanvas::new(std::io::stdout()).context("unable to create CrosstermCanvas")?;

    Editor::new(&mut canvas)
        .context("unable to initialise Editor")?
        .consume(map_crossterm_event_stream())
        .await
        .context("error during input event stream consumption")?;

    Ok(())
}
