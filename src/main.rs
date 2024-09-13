use std::cell::RefCell;
use std::collections::linked_list;
use std::io::{stdin, stdout, Write};
use std::rc::{Rc, Weak};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
struct Node {
    value: char,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, value: char) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }
    }

    fn push_prev(&mut self, value: char) {
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

    fn pop_front(&mut self) -> Option<char> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    fn pop_prev(&mut self) -> Option<char> {
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

    fn read_head(&self) -> Option<char> {
        self.head.as_ref().map(|node| node.borrow().value)
    }

    fn get_next(&self, current: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        current.borrow().next.as_ref().map(Rc::clone)
    }

    fn get_prev(&self, current: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        current.borrow().prev.as_ref().and_then(Weak::upgrade)
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
                    nodes.push_prev(c);
                }
                Key::Backspace => {
                    nodes.pop_prev();
                }
                Key::Esc => {
                    break;
                }
                _ => {}
            }
        }

        print!("{}", "\x1b[1;1H");
        if let Some(head) = &nodes.head {
            let mut current = Rc::clone(head);
            loop {
                print!("{} ", current.borrow().value);
                match nodes.get_next(&current) {
                    Some(next) => current = next,
                    None => break,
                }
            }
        }
        println!();
    }
}
