use crate::session::TypingSession;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::{self, Result, Write};

pub struct HandleInput;

impl HandleInput {
    pub fn handle_typing(session: &mut TypingSession, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char(c) => Self::handle_char_input(session, c)?,
            KeyCode::Backspace => Self::handle_backspace(session)?,
            _ => {}
        }
        Ok(())
    }

    pub fn handle_char_input(session: &mut TypingSession, c: char) -> Result<()> {
        let target_text: Vec<char> = session.target_text.chars().collect();
        let current_index = session.user_input.len();

        if current_index < target_text.len() {
            let expected_char = target_text[current_index];

            if c == expected_char {
                session.stats.correct_chars += 1;
            } else {
                session.stats.wrong_chars += 1;
            }
        }
        session.user_input.push(c);
        print!("{}", c);
        io::stdout().flush()?;
        Ok(())
    }

    pub fn handle_backspace(session: &mut TypingSession) -> Result<()> {
        if !session.user_input.is_empty()
            && let Some(last_char) = session.user_input.pop()
        {
            let current_index = session.user_input.len();
            let target_char = session.target_text.chars().nth(current_index).unwrap();

            if last_char == target_char {
                session.stats.correct_chars = session.stats.correct_chars.saturating_sub(1);
            } else {
                session.stats.wrong_chars = session.stats.wrong_chars.saturating_sub(1);
            }
            print!("\u{0008} \u{0008}");
            io::stdout().flush()?;
        }
        Ok(())
    }
}
