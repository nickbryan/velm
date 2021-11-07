#![warn(clippy::all, clippy::pedantic)]

use anyhow::Error;
use backtrace::Backtrace;
use crossterm::{style::Print, terminal::LeaveAlternateScreen};
use std::panic::{self, PanicInfo};
use velm_core::Editor;
use velm_tui::{map_crossterm_event_stream, CrosstermCanvas};

#[tokio::main]
async fn main() -> Result<(), Error> {
    use anyhow::Context;

    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    let mut canvas =
        CrosstermCanvas::new(std::io::stdout()).context("unable to create CrosstermCanvas")?;

    Editor::new(&mut canvas)
        .context("unable to initialise Editor")?
        .consume(map_crossterm_event_stream())
        .await
        .context("error during input event stream consumption")?;

    Ok(())
}

fn panic_hook(info: &PanicInfo<'_>) {
    let location = info.location().unwrap();

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

    crossterm::terminal::disable_raw_mode().expect("unable to disable raw mode");
    crossterm::execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        Print(format!(
            "thread '<unnamed>' panicked at '{}', {}\n\r{}",
            msg, location, stacktrace
        )),
    )
    .unwrap();
}
