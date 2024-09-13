use std::cell::RefCell;
use std::collections::linked_list;
use std::io::{stdin, stdout, Write};
use std::rc::{Rc, Weak};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;struct Node {
    value: char,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    cursor: Option<Rc<RefCell<Node>>>,
}
enum CursorDirection {
    Left,
    Right,
}
impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            cursor: None,
        }
    }
    fn push_back(&mut self, value: char) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }
    fn pop_back(&mut self) -> Option<char> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(prev) => {
                    let new_tail = prev.upgrade().unwrap();
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }
    fn move_cursor(&mut self, direction: CursorDirection) {
        if let Some(current_cursor) = &self.cursor {
            let next_cursor = match direction {
                CursorDirection::Left => self.get_prev(current_cursor),
                CursorDirection::Right => self.get_next(current_cursor),
            };
            if let Some(next) = next_cursor {
                self.cursor = Some(next);
            }
        }
    }
    fn read_head(&self) -> Option<char> {
        self.head.as_ref().map(|node| node.borrow().value)
    }
    fn get_next(&self, current: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        current.borrow().next.as_ref().cloned()
    }
    fn get_prev(&self, current: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        current.borrow().prev.as_ref().and_then(Weak::upgrade)
    }
    fn display(&self) -> std::io::Result<()> {
        // Clear the screen and move cursor to top-left corner
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));

        if let Some(head) = &self.head {
            let mut current = Rc::clone(head);
            loop {
                // Print the current node's value
                print!("{} ", current.borrow().value);
                std::io::stdout().flush()?;

                // Show cursor if this is the current cursor position
                if self.cursor.as_ref().map_or(false, |c| Rc::ptr_eq(c, &current)) {
                    print!("{}", cursor::Show);
                   std::io::stdout().flush()?;
                }

                // Move to next node or break if at the end
                match self.get_next(&current) {
                    Some(next) => current = next,
                    None => break,
                }
            }
        }

        println!();
        std::io::stdout().flush()?;

        Ok(())
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();
    let mut nodes = LinkedList::new();

    stdout.flush().unwrap();

    loop {
        if let Some(Ok(key)) = keys.next() {
            match key {
                Key::Char(c) => {
                    nodes.push_back(c);
                }
                Key::Backspace => {
                    nodes.pop_back();
                }
                Key::Esc => {
                    break;
                }
                _ => {}
            }
        }
        nodes.display().unwrap();
    }
}
