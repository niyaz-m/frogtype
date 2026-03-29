mod session;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{
    self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode,
};
use crossterm::{execute, queue};
use std::io::{self, Result, Stdout, Write};
use std::time::Duration;

use crate::session::TypingSession;

fn main() -> Result<()> {
    setup_terminal()?;

    let mut stdout = io::stdout();
    let target_text = "So beautiful, the space between. A painful reminder and a terrible dream.\n";
    let mut session = session::TypingSession::new(&target_text);

    loop {
        if session.start_time.elapsed() >= session.duration {
            session.is_finished = true;
        }

        draw_ui(&mut stdout, &session)?;

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if session.is_finished {
                    //if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                    //}
                } else {
                    handle_typing(&mut session, key)?;
                }
            }
        }
    }

    cleanup_terminal(stdout)?;

    let stats = format!(
        "WPM: {:.2} | Accuracy: {:.2}%",
        session.wpm(),
        session.accuracy()
    );
    println!("{}", stats);
    println!("Time's up!");
    println!("Text: {}", session.user_input);
    Ok(())
}

fn draw_ui(stdout: &mut Stdout, session: &TypingSession) -> Result<()> {
    let (width, height) = terminal::size()?;

    let x = (width / 2).saturating_sub(session.target_text.len() as u16 / 2);
    let y = height / 2;

    queue!(
        stdout,
        Clear(ClearType::All),
        MoveTo(x, y),
        SetForegroundColor(Color::Cyan),
        Print(session.target_text.clone())
    )?;

    queue!(
        stdout,
        MoveTo(x, y + 1),
        SetForegroundColor(Color::Blue),
        Print(session.user_input.clone())
    )?;

    stdout.flush()?;
    Ok(())
}

fn handle_typing(session: &mut TypingSession, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char(c) => {
            session.user_input.push(c);
            print!("{}", c);
            io::stdout().flush()?;
        }
        KeyCode::Backspace => {
            if !session.user_input.is_empty() {
                session.user_input.pop();
                print!("\u{0008} \u{0008}");
                io::stdout().flush()?;
            }
        }
        _ => {}
    }
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
