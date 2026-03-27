use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::queue;
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{
    self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
    enable_raw_mode,
};
use std::io::{self, Result, Stdout, Write};
use std::time::{Duration, Instant};

struct Characters {
    typed_chars: i8,
    correct_chars: i8,
    wrong_chars: i8,
}

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut text = String::new();
    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    let mut stdout = io::stdout();
    let text_to_print = "Hello, world! I like guys!\n";

    setup_terminal()?;

    let (width, height) = terminal::size()?;

    let x = (width / 2).saturating_sub(text_to_print.len() as u16 / 2);
    let y = height / 2;

    queue!(
        stdout,
        Clear(ClearType::All),
        MoveTo(x, y),
        SetForegroundColor(Color::Cyan),
        Print(text_to_print)
    )?;

    stdout.flush()?;

    queue!(stdout, MoveTo(x, y + 1), SetForegroundColor(Color::Blue))?;

    stdout.flush()?;

    loop {
        if start_time.elapsed() >= timeout {
            break;
        }

        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        text.push(c);
                        print!("{}", c);
                        io::stdout().flush()?;
                    }
                    KeyCode::Backspace => {
                        if !text.is_empty() {
                            text.pop();
                            print!("\u{0008} \u{0008}");
                            io::stdout().flush()?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    cleanup_terminal(stdout)?;

    println!("Time's up!");

    let mut typing_session = Characters {
        typed_chars: text.len() as i8,
        correct_chars: 0,
        wrong_chars: 0,
    };

    for (_, (text, text_to_print)) in text.chars().zip(text_to_print.chars()).enumerate() {
        if text != text_to_print {
            typing_session.wrong_chars += 1;
        } else {
            typing_session.correct_chars += 1;
        }
    }

    println!("wrong chars: {}", typing_session.wrong_chars);
    println!("correct chars: {}", typing_session.correct_chars);

    let accuracy = (typing_session.correct_chars as f64 / text.len() as f64) * 100.0;
    println!("Accuracy: {:.2}", accuracy);

    let current_wpm = wpm_calc(text.clone(), start_time);
    println!(
        "WPM: {:.0} | Time: {}s",
        current_wpm,
        start_time.elapsed().as_secs()
    );

    println!("Text count: {}", typing_session.typed_chars);
    println!("Term size: {}, {}", width, height);
    println!("Text: {}", text);
    Ok(())
}

fn wpm_calc(text: String, start_time: Instant) -> f64 {
    let text_len = text.len();

    let elapsed_secs = start_time.elapsed().as_secs_f64();
    if elapsed_secs < 1.0 {
        return 0.0;
    }

    let minutes = elapsed_secs / 60.0;
    let words = text_len as f64 / 5.0;

    words / minutes
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
