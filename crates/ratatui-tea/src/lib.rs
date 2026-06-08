//! Optional Bubble Tea-style app-loop shell for ratatui.
//!
//! This crate does not replace ratatui's renderer. Models still draw with a
//! normal ratatui [`Frame`].

use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, SendError, Sender};

use ratatui::backend::Backend;
use ratatui::{CompletedFrame, Frame, Terminal};

/// Application model using Elm/Bubble Tea-style update and view methods.
pub trait Model {
    /// Message type handled by this model.
    type Msg: Send + 'static;

    /// Initial side effects/messages.
    fn init(&mut self) -> Cmd<Self::Msg> {
        Cmd::none()
    }

    /// Applies a message and returns follow-up commands/messages.
    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg>;

    /// Renders the model through ratatui.
    fn view(&self, frame: &mut Frame<'_>);
}

/// Immediate command messages for the MVP app-loop shell.
///
/// Async work, timers, and one-shot ticks belong to the next milestone. This
/// type already models the command boundary so app code does not need to change
/// when richer command executors arrive.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cmd<M> {
    messages: Vec<M>,
}

impl<M> Cmd<M> {
    /// Creates a command with no messages.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Creates a command that emits one message.
    #[must_use]
    pub fn message(message: M) -> Self {
        Self {
            messages: vec![message],
        }
    }

    /// Batches commands in order.
    #[must_use]
    pub fn batch(commands: impl IntoIterator<Item = Self>) -> Self {
        Self {
            messages: commands
                .into_iter()
                .flat_map(|command| command.messages)
                .collect(),
        }
    }

    /// Returns whether the command contains no messages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Consumes the command and returns its messages.
    #[must_use]
    pub fn into_messages(self) -> Vec<M> {
        self.messages
    }
}

impl<M> Default for Cmd<M> {
    fn default() -> Self {
        Self::none()
    }
}

/// A running program around a [`Model`].
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Program<M: Model> {
    model: M,
}

impl<M: Model> Program<M> {
    /// Creates a program from a model.
    #[must_use]
    pub const fn new(model: M) -> Self {
        Self { model }
    }

    /// Returns the model.
    #[must_use]
    pub const fn model(&self) -> &M {
        &self.model
    }

    /// Returns the model mutably.
    #[must_use]
    pub const fn model_mut(&mut self) -> &mut M {
        &mut self.model
    }

    /// Runs `Model::init` and processes any immediate command messages.
    pub fn init(&mut self) {
        let command = self.model.init();
        self.process(command);
    }

    /// Sends one message to the model and processes immediate follow-up messages.
    pub fn send(&mut self, message: M::Msg) {
        let command = self.model.update(message);
        self.process(command);
    }

    /// Draws the model with a normal ratatui terminal.
    pub fn draw<'a, B: Backend>(
        &self,
        terminal: &'a mut Terminal<B>,
    ) -> Result<CompletedFrame<'a>, B::Error> {
        terminal.draw(|frame| self.model.view(frame))
    }

    fn process(&mut self, command: Cmd<M::Msg>) {
        let mut queue = VecDeque::from(command.into_messages());

        while let Some(message) = queue.pop_front() {
            queue.extend(self.model.update(message).into_messages());
        }
    }
}

/// Cloneable handle for sending messages into an external program loop.
#[derive(Debug, Clone)]
pub struct ProgramHandle<M> {
    sender: Sender<M>,
}

impl<M> ProgramHandle<M> {
    /// Sends a message to the paired receiver.
    pub fn send(&self, message: M) -> Result<(), SendError<M>> {
        self.sender.send(message)
    }
}

/// Creates a message handle and receiver pair for external event loops.
#[must_use]
pub fn channel<M>() -> (ProgramHandle<M>, Receiver<M>) {
    let (sender, receiver) = mpsc::channel();
    (ProgramHandle { sender }, receiver)
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui_bubbletea_theme::BubbleTheme;

    use super::{Cmd, Model, Program, channel};

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Counter {
        count: i32,
        theme: BubbleTheme,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    enum Msg {
        Increment,
        Decrement,
    }

    impl Model for Counter {
        type Msg = Msg;

        fn init(&mut self) -> Cmd<Self::Msg> {
            Cmd::message(Msg::Increment)
        }

        fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg> {
            match msg {
                Msg::Increment => self.count += 1,
                Msg::Decrement => self.count -= 1,
            }

            Cmd::none()
        }

        fn view(&self, frame: &mut ratatui::Frame<'_>) {
            frame.render_widget(
                self.theme.paragraph(format!("count: {}", self.count)),
                frame.area(),
            );
        }
    }

    #[test]
    fn program_processes_init_and_messages() {
        let mut program = Program::new(Counter {
            count: 0,
            theme: BubbleTheme::default(),
        });

        program.init();
        assert_eq!(program.model().count, 1);

        program.send(Msg::Increment);
        program.send(Msg::Decrement);
        assert_eq!(program.model().count, 1);
    }

    #[test]
    fn command_batch_preserves_order() {
        let command = Cmd::batch([Cmd::message(1), Cmd::message(2), Cmd::none()]);

        assert_eq!(command.into_messages(), vec![1, 2]);
    }

    #[test]
    fn program_draws_with_ratatui_terminal() -> Result<(), Box<dyn std::error::Error>> {
        let program = Program::new(Counter {
            count: 7,
            theme: BubbleTheme::default(),
        });
        let backend = TestBackend::new(8, 1);
        let mut terminal = Terminal::new(backend)?;

        program.draw(&mut terminal)?;

        let buffer = terminal.backend().buffer();
        let rendered = (0..8).map(|x| buffer[(x, 0)].symbol()).collect::<String>();
        assert_eq!(rendered, "count: 7");

        Ok(())
    }

    #[test]
    fn handle_sends_messages_to_receiver() {
        let (handle, receiver) = channel();

        handle.send(Msg::Increment).expect("receiver is open");

        assert_eq!(receiver.recv().expect("message exists"), Msg::Increment);
    }
}
