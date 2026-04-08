use std::time::{Duration, Instant};

pub struct CharacterStats {
    pub correct_chars: usize,
    pub wrong_chars: usize,
}

#[derive(PartialEq, Debug)]
pub enum SessionState {
    Waiting,
    Running,
    Finished,
}

pub struct TypingSession {
    pub target_text: String,
    pub user_input: String,
    pub stats: CharacterStats,
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
            stats: CharacterStats {
                correct_chars: 0,
                wrong_chars: 0,
            },
            state: SessionState::Waiting,
            start_time: Some(Instant::now()),
            duration: Duration::from_secs(5),
            final_time: Some(Duration::from_secs(5)),
        }
    }

    pub fn correct_chars_count(&self) -> usize {
        self.user_input
            .chars()
            .zip(self.target_text.chars())
            .filter(|(u, t)| u == t)
            .count()
    }

    pub fn accuracy(&self) -> f64 {
        let total = self.user_input.len();
        if total == 0 {
            return 100.0;
        }

        let correct = self.correct_chars_count() as f64;
        (correct / total as f64) * 100.0
    }

    pub fn wpm(&self) -> (f64, f64) {
        let elapsed_secs = match self.state {
            SessionState::Waiting => return (0.0, 0.0),
            SessionState::Running => self.start_time.unwrap().elapsed().as_secs_f64(),
            SessionState::Finished => self.final_time.unwrap().as_secs_f64(),
        };

        let text_len = self.user_input.len();
        let correct_chars = self.correct_chars_count();

        let net_words = correct_chars as f64 / 5.0;
        let words = text_len as f64 / 5.0;

        let minutes = elapsed_secs / 60.0;
        let net_wpm = net_words / minutes;
        let raw_wpm = words / minutes;

        (net_wpm, raw_wpm)
    }

    pub fn time_remaining(&self) -> f64 {
        match self.state {
            SessionState::Waiting => self.duration.as_secs_f64(),
            SessionState::Running => {
                let elapsed = self.start_time.unwrap().elapsed();
                self.duration.saturating_sub(elapsed).as_secs_f64()
            }
            SessionState::Finished => 0.0,
        }
    }

    pub fn reset_sesssion(&mut self) {
        self.user_input.clear();
        self.state = SessionState::Waiting;
        self.start_time = None;
        self.final_time = None;
        self.stats.correct_chars = 0;
        self.stats.wrong_chars = 0;
    }
}
