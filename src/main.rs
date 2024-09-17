use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();
    stdout.flush().unwrap();

    loop {
        print!("\x1b[1;1H");
        if let Some(Ok(key)) = keys.next() {
            match key {
                Key::Char(c) => {
                    print!("{}", c);
                }
                Key::Backspace => {}
                Key::Esc => {
                    break;
                }
                _ => {}
            }
        }
        stdout.flush().unwrap();
    }
}
