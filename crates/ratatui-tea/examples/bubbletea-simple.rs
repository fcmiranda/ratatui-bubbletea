//! Port of Bubble Tea's `examples/simple` idea.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui_bubbletea_theme::BubbleTheme;
use ratatui_tea::{Cmd, Model, Program};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Counter {
    count: i32,
    theme: BubbleTheme,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Msg {
    Increment,
    Decrement,
}

impl Model for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg> {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }

        Cmd::none()
    }

    fn view(&self, frame: &mut ratatui::Frame<'_>) {
        frame.render_widget(
            self.theme
                .paragraph_in_block(format!("count: {}", self.count), "Simple counter"),
            frame.area(),
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program = Program::new(Counter {
        count: 0,
        theme: BubbleTheme::default(),
    });
    let backend = TestBackend::new(32, 3);
    let mut terminal = Terminal::new(backend)?;

    program.send(Msg::Increment);
    program.send(Msg::Increment);
    program.send(Msg::Decrement);
    program.draw(&mut terminal)?;

    Ok(())
}
