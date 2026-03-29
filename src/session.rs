use std::time::{Duration, Instant};

struct CharacterStats {
    pub correct_chars: usize,
    pub wrong_chars: usize,
}

pub struct TypingSession {
    pub target_text: String,
    pub user_input: String,
    pub start_time: Instant,
    pub duration: Duration,
    pub is_finished: bool,
}

impl TypingSession {
    pub fn new(text: &str) -> Self {
        Self {
            target_text: text.to_string(),
            user_input: String::new(),
            start_time: Instant::now(),
            duration: Duration::from_secs(5),
            is_finished: false,
        }
    }

    pub fn accuracy(&self) -> f64 {
        let mut chars = CharacterStats {
            correct_chars: 0,
            wrong_chars: 0,
        };
        for (_, (target_text, text_to_print)) in self
            .user_input
            .chars()
            .zip(self.target_text.chars())
            .enumerate()
        {
            if target_text != text_to_print {
                chars.wrong_chars += 1;
            } else {
                chars.correct_chars += 1;
            }
        }

        let correct_chars = chars.correct_chars as f64;
        let user_input_len = self.user_input.len() as f64;
        let accuracy = (correct_chars / user_input_len) * 100.0;
        accuracy
    }

    pub fn wpm(&self) -> f64 {
        let text_len = self.user_input.len();

        let elapsed_secs = self.start_time.elapsed().as_secs_f64();
        if elapsed_secs < 1.0 {
            return 0.0;
        }

        let minutes = elapsed_secs / 60.0;
        let words = text_len as f64 / 5.0;
        let wpm = words / minutes;
        wpm
    }
}
