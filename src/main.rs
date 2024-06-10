mod config;
mod screen;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io::stdout;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let arguments = config::Args::parse();
    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut screen = screen::Screen::new_menu();
    if let Some(config::Command::Add {
        question,
        choices,
        answer,
    }) = arguments.command
    {
        let mut questions = screen::Questions::load_from_file();
        let question = config::add_to_question(question, choices, answer);
        questions.add_question(question);
        std::process::exit(0);
    }

    let res = run(&mut t, &mut screen);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;

    res?;
    Ok(())
}
fn run<B: Backend>(t: &mut Terminal<B>, screen: &mut screen::Screen) -> anyhow::Result<()> {
    loop {
        t.draw(|f| screen.draw(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Down => screen.question.down(),
                KeyCode::Up => screen.question.up(),
                KeyCode::Enter => match screen.state {
                    screen::State::Menu => screen.start_game(),
                    screen::State::Game => screen.update(),
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}
