use tokio::time::Duration;
use velm_core::Editor;
use velm_tui::CrosstermEventStream;

#[tokio::main]
async fn main() {
    Editor::new(Box::pin(CrosstermEventStream::new(Duration::from_millis(
        250,
    ))))
    .run()
    .await;
}
