use std::time::Duration;

use crossterm::{
    event::{poll, read},
    Result,
};

fn main() -> Result<bool> {
    loop {
        if poll(Duration::from_millis(100))? {
            // It's guaranteed that `read` won't block, because `poll` returned
            // `Ok(true)`.
            println!("{:?}", read()?);
        } else {
            // Timeout expired, no `Event` is available
            println!("no input");
        }
    }
}
