//! Port of Bubble Tea's `examples/sequence` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui_bubbletea_theme::BubbleTheme;
use ratatui_tea::{Cmd, Model, Program};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct LogModel {
    events: Vec<&'static str>,
    theme: BubbleTheme,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Msg {
    Boot,
    Connect,
    Ready,
}

impl Model for LogModel {
    type Msg = Msg;

    fn init(&mut self) -> Cmd<Self::Msg> {
        Cmd::sequence([
            Cmd::message(Msg::Boot),
            Cmd::once(|| Msg::Connect),
            Cmd::message(Msg::Ready),
        ])
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg> {
        match msg {
            Msg::Boot => self.events.push("boot"),
            Msg::Connect => self.events.push("connect"),
            Msg::Ready => self.events.push("ready"),
        }

        Cmd::none()
    }

    fn view(&self, frame: &mut ratatui::Frame<'_>) {
        frame.render_widget(
            self.theme
                .paragraph_in_block(self.events.join(" → "), "Sequenced commands"),
            frame.area(),
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program = Program::new(LogModel {
        events: Vec::new(),
        theme: BubbleTheme::default(),
    });
    let backend = TestBackend::new(40, 3);
    let mut terminal = Terminal::new(backend)?;

    program.init();
    program.draw(&mut terminal)?;

    Ok(())
}
