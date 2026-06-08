//! Port of Bubble Tea's `examples/simple` idea.

use std::io;

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
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
            self.theme.paragraph_in_block(
                format!(
                    "count: {}\n\n↑/+ increment  ↓/- decrement  q quit",
                    self.count
                ),
                "Simple counter",
            ),
            frame.area(),
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let mut program = Program::new(Counter {
        count: 0,
        theme: BubbleTheme::default(),
    });

    let result = run(&mut terminal, &mut program);
    restore_terminal(terminal)?;
    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    program: &mut Program<Counter>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        program.draw(terminal)?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => break,
            KeyCode::Char('+' | '=') | KeyCode::Up => program.send(Msg::Increment),
            KeyCode::Char('-') | KeyCode::Down => program.send(Msg::Decrement),
            _ => {}
        }
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
