//! Port of Bubble Tea's `examples/sequence` idea.

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct LogModel {
    events: Vec<&'static str>,
    theme: BubbleTheme,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Msg {
    Replay,
    Clear,
    Boot,
    Connect,
    Ready,
}

impl Model for LogModel {
    type Msg = Msg;

    fn init(&mut self) -> Cmd<Self::Msg> {
        sequence_messages()
    }

    fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg> {
        match msg {
            Msg::Replay => return Cmd::sequence([Cmd::message(Msg::Clear), sequence_messages()]),
            Msg::Clear => self.events.clear(),
            Msg::Boot => self.events.push("boot"),
            Msg::Connect => self.events.push("connect"),
            Msg::Ready => self.events.push("ready"),
        }

        Cmd::none()
    }

    fn view(&self, frame: &mut ratatui::Frame<'_>) {
        frame.render_widget(
            self.theme.paragraph_in_block(
                format!("{}\n\nr replay sequence  q quit", self.events.join(" → ")),
                "Sequenced commands",
            ),
            frame.area(),
        );
    }
}

fn sequence_messages() -> Cmd<Msg> {
    Cmd::sequence([
        Cmd::message(Msg::Boot),
        Cmd::once(|| Msg::Connect),
        Cmd::message(Msg::Ready),
    ])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let mut program = Program::new(LogModel {
        events: Vec::new(),
        theme: BubbleTheme::default(),
    });

    program.init();
    let result = run(&mut terminal, &mut program);
    restore_terminal(terminal)?;
    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    program: &mut Program<LogModel>,
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
            KeyCode::Char('r') => program.send(Msg::Replay),
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
