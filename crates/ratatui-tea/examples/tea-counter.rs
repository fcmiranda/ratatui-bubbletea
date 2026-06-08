use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui_bubbletea_theme::BubbleTheme;
use ratatui_tea::{Cmd, Model, Program};

struct Counter {
    count: i32,
    theme: BubbleTheme,
}

enum Msg {
    Increment,
}

impl Model for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg> {
        match msg {
            Msg::Increment => self.count += 1,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program = Program::new(Counter {
        count: 0,
        theme: BubbleTheme::default(),
    });
    let backend = TestBackend::new(16, 1);
    let mut terminal = Terminal::new(backend)?;

    program.send(Msg::Increment);
    program.draw(&mut terminal)?;

    Ok(())
}
