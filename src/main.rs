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

fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut text = String::new();
    let start_time = Instant::now();
    let timeout = Duration::from_secs(5);

    let mut stdout = io::stdout();
    let text_to_print = "Hello, world!";

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
    println!("term size: {}, {}", width, height);
    println!("Text: {}", text);
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
