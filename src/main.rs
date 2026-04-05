mod input;
mod session;
mod ui;

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use std::{
    io::{self, Result, Stdout},
    time::{Duration, Instant},
};

use crate::{input::HandleInput, session::SessionState, ui::draw_ui};

fn main() -> Result<()> {
    setup_terminal()?;

    let mut stdout = io::stdout();
    let target_text = "So beautiful, the space between. A painful reminder and a terrible dream.\n";
    let mut session = session::TypingSession::new(target_text);

    loop {
        draw_ui(&mut stdout, &session)?;

        if session.state == SessionState::Running {
            let elapsed = session.start_time.unwrap().elapsed();
            if elapsed >= session.duration {
                session.state = SessionState::Finished;
                session.final_time = Some(elapsed);
            }
        }

        if event::poll(Duration::from_millis(10))?
            && let Event::Key(key) = event::read()?
        {
            match session.state {
                SessionState::Waiting => {
                    if let KeyCode::Char(c) = key.code {
                        session.state = SessionState::Running;
                        session.start_time = Some(Instant::now());
                        session.user_input.push(c);
                    }
                }

                SessionState::Running => HandleInput::handle_typing(&mut session, key)?,

                SessionState::Finished => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => session.reset_sesssion(),
                    _ => {}
                },
            }
        }
    }

    cleanup_terminal(stdout)?;

    Ok(())
}

fn setup_terminal() -> io::Result<Stdout> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
        SetForegroundColor(Color::White)
    )?;
    Ok(stdout)
}

fn cleanup_terminal(mut stdout: Stdout) -> io::Result<()> {
    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
