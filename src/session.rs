use std::time::{Duration, Instant};

struct CharacterStats {
    pub correct_chars: usize,
    pub wrong_chars: usize,
}

#[derive(Debug)]
pub enum SessionState {
    Waiting,
    Running,
    Finished,
}

pub struct TypingSession {
    pub target_text: String,
    pub user_input: String,
    pub state: SessionState,
    pub start_time: Option<Instant>,
    pub duration: Duration,
    pub final_time: Option<Duration>,
}

impl TypingSession {
    pub fn new(text: &str) -> Self {
        Self {
            target_text: text.to_string(),
            user_input: String::new(),
            state: SessionState::Waiting,
            start_time: Some(Instant::now()),
            duration: Duration::from_secs(5),
            final_time: Some(Duration::from_secs(5)),
        }
    }

    pub fn accuracy(&self) -> f64 {
        let mut chars = CharacterStats {
            correct_chars: 0,
            wrong_chars: 0,
        };
        for (_, (user_input, target_text)) in self
            .user_input
            .chars()
            .zip(self.target_text.chars())
            .enumerate()
        {
            if user_input != target_text {
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
        let elapsed_secs = match self.state {
            SessionState::Waiting => return 0.0,
            SessionState::Running => self.start_time.unwrap().elapsed().as_secs_f64(),
            SessionState::Finished => self.final_time.unwrap().as_secs_f64(),
        };

        let text_len = self.user_input.len();
        let minutes = elapsed_secs / 60.0;
        let words = text_len as f64 / 5.0;
        let wpm = words / minutes;
        wpm
    }

    pub fn reset_sesssion(&mut self) {
        self.user_input.clear();
        self.state = SessionState::Waiting;
        self.start_time = None;
        self.final_time = None;
    }
}
