use crossterm::{
    cursor::MoveTo,
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
        "WPM: {:.2} | RAW_WPM: {:.2} | Accuracy: {:.2}% | Correct chars: {} | Wrong chars: {}",
        net_wpm,
        raw_wpm,
        session.accuracy(),
        session.stats.correct_chars,
        session.stats.wrong_chars
    );

    match session.state {
        SessionState::Waiting | SessionState::Running => {
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

            queue!(
                stdout,
                MoveTo(x, y + 5),
                SetForegroundColor(Color::Yellow),
                Print(stats),
            )?;
        }
        SessionState::Finished => {
            queue!(stdout, Clear(ClearType::All), MoveTo(x, y), Print(stats),)?;
        }
    }

    stdout.flush()?;
    Ok(())
}
