use crate::EventStream;
use tokio_stream::StreamExt;

pub struct Editor {
    event_stream: EventStream,
}

impl Editor {
    pub fn new(event_stream: EventStream) -> Self {
        Self { event_stream }
    }

    pub async fn run(&mut self) {
        while let Some(event) = self.event_stream.next().await {
            println!("{:?}", event);
        }
    }
}
