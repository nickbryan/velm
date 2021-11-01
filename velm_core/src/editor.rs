use crate::EventStream;
use tokio_stream::StreamExt;

pub struct Editor {
    event_stream: EventStream,
}

impl Editor {
    #[must_use]
    pub fn new(event_stream: EventStream) -> Self {
        Self { event_stream }
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(event) = self.event_stream.next() => {
                    println!("{:?}", event);
                }
                else => break,
            }
        }
    }
}
