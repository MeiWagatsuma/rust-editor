use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

fn main() -> Result<()> {
    let mut stdout = stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )?;

    for y in 0..40 {
        for x in 0..150 {
            if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                // in this loop we are more efficient by not flushing the buffer.
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("█".magenta())
                )?;
            }
        }
    }
    stdout.flush()?;
    thread::sleep(Duration::from_secs(1));

    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
