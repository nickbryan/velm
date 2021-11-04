use crate::communication::{Command, Message};
use crate::component::{Component, Window};
use crate::render::{View, Viewport};
use crate::{Canvas, Event, EventStream, Key};
use anyhow::{Error, Result};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

/// `Editor` is the entry point into the application and is responsible for orchestrating
/// communication between `Component`s.
pub struct Editor<'a, VC, C>
where
    VC: View + Component,
    C: Canvas,
{
    msg_tx: mpsc::Sender<Message>,
    msg_rx: mpsc::Receiver<Message>,
    root_component: VC,
    should_quit: bool,
    viewport: Viewport<'a, C>,
}

impl<'a, C> Editor<'a, Window, C>
where
    C: Canvas,
{
    /// Create a new editor using the default `View` `Component` and the given `Canvas`.
    ///
    /// # Errors
    ///
    /// Can error while creating the `Viewport` if the underlying `Canvas` has IO issues.
    pub fn new(canvas: &'a mut C) -> Result<Self> {
        use anyhow::Context;

        let (msg_tx, msg_rx) = mpsc::channel(1);

        Ok(Self {
            msg_rx,
            msg_tx,
            root_component: Window {},
            should_quit: false,
            viewport: Viewport::new(canvas).context("unable to initialise Viewport")?,
        })
    }
}

impl<'a, VC, C> Editor<'a, VC, C>
where
    VC: Component + View,
    C: Canvas,
{
    /// Consume the given `EventStream` to run/drive the Editor.
    ///
    /// # Errors
    ///
    /// Will return `Err` when a message was received on the `err_tx`.
    pub async fn consume(&mut self, mut event_stream: EventStream) -> Result<()> {
        use anyhow::Context;

        // TODO: figure out the buffer size of these channels.
        let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(1);
        let (err_tx, mut err_rx) = mpsc::channel::<Error>(1);

        let msg_tx = self.msg_tx.clone();
        tokio::spawn(async move {
            while let Some(event) = event_stream.next().await {
                match event {
                    Event::KeyPressed(Key::Esc) => {
                        msg_tx
                            .send(Message::Quit)
                            .await
                            .expect("unable to send msg on closed msg_tx channel");
                    }
                    Event::KeyPressed(Key::Char(ch)) => {
                        msg_tx
                            .send(Message::InsertChar(ch))
                            .await
                            .expect("unable to send msg on closed msg_tx channel");
                    }
                    _ => (),
                }
            }
        });

        let msg_tx = self.msg_tx.clone();
        tokio::spawn(async move {
            while let Some(cmd) = cmd_rx.recv().await {
                let msg_tx = msg_tx.clone();

                // Each command is spawned in its own async block as they may take time to complete.
                tokio::spawn(async move {
                    msg_tx
                        .send(cmd())
                        .await
                        .expect("unable to send cmd result on closed msg_tx channel");
                });
            }
        });

        while !self.should_quit {
            tokio::select! {
                Some(msg) = self.msg_rx.recv() => {
                    if let Message::Quit = msg {
                        self.should_quit = true;
                    }

                    if let Some(cmd) = self.root_component.update(msg) {
                        cmd_tx.send(cmd).await.expect("unable to send on closed cmd_tx channel");
                    }

                    if let Err(e) = self.viewport.render(&self.root_component).context("rendering error occurred") {
                        err_tx.send(e).await.expect("unable to send on closed err_tx channel");
                    }
                }
                Some(e) = err_rx.recv() => {
                    return Err(e);
                }
                else => break,
            }
        }

        Ok(())
    }
}
