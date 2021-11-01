use crate::{
    communication::{Command, Message},
    Event, EventStream, Key,
};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

pub trait Component {
    fn update(&mut self, msg: Message) -> Option<Command>;
}

pub struct WindowComponent;

impl Component for WindowComponent {
    fn update(&mut self, msg: Message) -> Option<Command> {
        println!("{:?}", msg);

        None
    }
}

pub struct Editor<C>
where
    C: Component,
{
    root_component: C,
    should_quit: bool,
}

impl Default for Editor<WindowComponent> {
    fn default() -> Self {
        Self {
            root_component: WindowComponent {},
            should_quit: false,
        }
    }
}

impl<C> Editor<C>
where
    C: Component,
{
    /// Consume the given EventStream to run/drive the Editor.
    pub async fn consume(&mut self, mut event_stream: EventStream) {
        let (msg_tx, mut msg_rx) = mpsc::channel(1);

        tokio::spawn(async move {
            while let Some(event) = event_stream.next().await {
                match event {
                    Event::KeyPressed(Key::Esc) => {
                        msg_tx.send(Message::Quit).await.unwrap_or_default();
                    }
                    Event::KeyPressed(Key::Char(ch)) => {
                        msg_tx
                            .send(Message::InsertChar(ch))
                            .await
                            .unwrap_or_default();
                    }
                    _ => (),
                }
            }
        });

        while !self.should_quit {
            tokio::select! {
                Some(msg) = msg_rx.recv() => {
                    if let Message::Quit = msg {
                        self.should_quit = true;
                    }

                    self.root_component.update(msg);
                }
                else => break,
            }
        }
    }
}
