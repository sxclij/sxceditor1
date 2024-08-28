use std::io::{stdin, stdout, Write};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let stdin = stdin();
    let mut keys = stdin.keys();

    stdout.flush().unwrap();

    loop {
        print!("{}", "\x1b[1;1H");
        if let Some(Ok(key)) = keys.next() {
            match key {
                Key::Char(c) => {
                    print!("{}", c);
                    stdout.flush().unwrap();
                },
                Key::Esc => {
                    break;
                },
                _ => {}
            }
        }
    }
}