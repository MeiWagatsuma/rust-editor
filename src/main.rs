use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

fn main() -> Result<()> {
    let stdout = stdout();
    let mut buffer = Buffer {
        stdout,
        column: 0,
        row: 0,
    };
    buffer.init();

    loop {
        if poll(Duration::from_millis(100))? {
            let event = read()?;
            if let Event::Key(key) = event {
                // キーコードと修飾キーに分解する
                let code = key.code;
                let modifiers = key.modifiers;

                // キーコードに応じて処理を分岐する
                match code {
                    KeyCode::Backspace => println!("Backspace pressed"),
                    KeyCode::Char(c) => buffer.insert(c),
                    _ => println!("Other key pressed: {:?}", code),
                }

                if !modifiers.is_empty() {
                    println!("Modifiers: {:?}", modifiers);
                }

                if code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
    buffer.render();
    thread::sleep(Duration::from_secs(1));

    buffer.quit();

    Ok(())
}

struct Buffer {
    stdout: Stdout,
    column: u16,
    row: u16,
}

impl Buffer {
    fn init(&mut self) {
        execute!(
            &self.stdout,
            EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All)
        );
    }

    fn render(&mut self) {
        self.stdout.flush();
    }

    fn insert(&mut self, input_char: char) {
        execute!(
            &self.stdout,
            cursor::MoveTo(self.column, self.row),
            style::Print(input_char)
        );
        self.column = &self.column + 1;
    }

    fn quit(self) {
        execute!(&self.stdout, LeaveAlternateScreen);
    }
}
