use std::io::{stdout, Stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn setup_ui() -> Terminal<TermionBackend<RawTerminal<Stdout>>> {
    let stdout = stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    Terminal::new(backend).unwrap()
}

pub fn draw_layout() {
    let mut term = setup_ui();
    term.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })
    .unwrap();
}
