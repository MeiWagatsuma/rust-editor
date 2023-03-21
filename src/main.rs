use std::{
    io::{stdout, Stdout, Write},
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
    let mut buffer = Buffer {
        stdout,
        column: 0,
        row: 0,
    };
    buffer.init();

    buffer.insert('h');
    buffer.insert('e');
    buffer.insert('l');
    buffer.insert('l');
    buffer.insert('o');

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
