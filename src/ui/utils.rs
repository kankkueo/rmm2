use std::io;
use termion::{event::Key, input::TermRead};
use tui::widgets::{ListItem, ListState};

pub fn keyin() -> Key {
    let stdin = io::stdin();
    for evt in stdin.keys() {
        match evt {
            Ok(x) =>  { return x; },
            Err(_e) => { return Key::Null; },
        }
    }
    Key::Null
}

pub struct StateList<'a> {
    pub items: Vec<ListItem<'a>>,
    pub state: ListState,
}

#[allow(dead_code)]
impl<'a> StateList<'a> {
    pub fn new() -> StateList<'a> {
        StateList {
            items: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn from(vec: Vec<String>) -> StateList<'a> {
        let mut items: Vec<ListItem> = Vec::new();
        for i in 0..vec.len() {
            items.push(ListItem::new(vec[i].clone()));
        }
         StateList {
            items: items ,
            state: ListState::default(),
        }       
    }

    pub fn update(&mut self, vec: Vec<String>) {
        for i in 0..vec.len() {
            self.items[i] = ListItem::new(vec[i].clone());
        }
    }

    pub fn select_next(&mut self) {
        match self.state.selected() {
            None => self.state.select(Some(0)),
            Some(x) =>  {
                if x < self.items.len() -1 {
                    self.state.select(Some(x + 1));
                }
            }
        }
    }

    pub fn select_prev(&mut self) {
        match self.state.selected() {
            None => self.state.select(Some(0)),
            Some(x) => {
                if x > 0 {
                    self.state.select(Some(x - 1));
                }
            }
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

fn fit_paragraph(src: &str, w: u16) -> String {
    let mut s = String::new();
    let mut line: u16 = 0;

    for i in src.split_whitespace() {
        if line + i.len() as u16  + 1 >= w {
            s.push('\n');
            line = 0;
        }

        s.push_str(i);
        s.push(' ');
        line += i.len() as u16 + 1;
    }

    s

}

pub fn fit_info(src: &str, w: u16) -> String {
    let mut s = String::new();

    for i in src.split('\n') {
        s.push('\n');
        s.push_str(&fit_paragraph(i, w));
    }

    s
}

