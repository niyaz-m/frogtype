use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use std::io::{Result, Stdout, Write};

use crate::session::{SessionState, TypingSession};

pub fn draw_ui(stdout: &mut Stdout, session: &TypingSession) -> Result<()> {
    let (width, height) = terminal::size()?;

    let x = (width / 2).saturating_sub(session.target_text.len() as u16 / 2);
    let y = height / 2;

    let (net_wpm, raw_wpm) = session.wpm();
    let stats = format!(
        "Time: {:.2} | WPM: {:.2} | RAW_WPM: {:.2} | Accuracy: {:.2}% | Correct chars: {} | Wrong chars: {}",
        session.time_remaining(),
        net_wpm,
        raw_wpm,
        session.accuracy(),
        session.stats.correct_chars,
        session.stats.wrong_chars
    );

    match session.state {
        SessionState::Waiting | SessionState::Running => {
            queue!(stdout, Clear(ClearType::All), MoveTo(x, y))?;

            for (i, target_char) in session.target_text.chars().enumerate() {
                let user_char = session.user_input.chars().nth(i);

                let color = match user_char {
                    None => Color::DarkGrey,
                    Some(c) if c == target_char => Color::White,
                    Some(_) => Color::Red,
                };

                queue!(stdout, SetForegroundColor(color), Print(target_char))?;
            }

            queue!(
                stdout,
                MoveTo(x, y + 5),
                SetForegroundColor(Color::Yellow),
                Print(stats),
                SetForegroundColor(Color::Reset),
            )?;

            let cursor_x = x + session.user_input.len() as u16;

            queue!(stdout, MoveTo(cursor_x, y), Show)?;
        }

        SessionState::Finished => {
            queue!(
                stdout,
                Clear(ClearType::All),
                MoveTo(x, y),
                SetForegroundColor(Color::Yellow),
                Hide,
                Print(stats),
            )?;
        }
    }

    stdout.flush()?;
    Ok(())
}
