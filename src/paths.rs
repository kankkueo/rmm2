use std::fs::read_dir;
use crate::files::read_datadir;

#[derive(Clone)]
pub struct Path {
    path: String,
}

impl Path {

    pub fn new() -> Path {
        Path {path: String::new()}
    }

    pub fn from(src: &str) -> Path {
        let mut s = String::new();

        for i in src.chars() {
            if  i == '\\' {
                s.push('/');
            }
            else {
                s.push(i);
            }
        }

       Path {path: s} 
    }

    pub fn push(&mut self, t: &str) -> Path {
        if self.path.ends_with('/') {
            self.path.push_str(&r_first(t));
        }
        else {
            self.path.push('/');
            self.path.push_str(&r_first(t));
        }
        self.clone()
    }

    pub fn push_p(&mut self, t: Path) -> Path {
        self.push(&t.path).clone()
    }

    pub fn is_dir(&self) -> bool {
        match read_dir(&self.path) {
            Err(_e) => false,
            Ok(_x) => true,
        }
    }

    pub fn previous(&self) -> Path {
        let items = self.items();
        let mut s = String::from("/");
        for i in 0..items.len() - 1 {
            s.push_str(&items[i]);
            s.push('/');
        }

        Path {path: s}
    }

    pub fn next(&mut self) -> Path {
        if self.is_dir() {
            let contents = read_datadir(&self).unwrap();
            if contents.len() == 1 {
                self.push(&contents[0]);
            }
        }
        self.clone()
    }

    pub fn items(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for i in self.path.split('/') {
            if i.len() > 0 {
                v.push(i.to_string());
            }
        }
        v
    }

    pub fn len(&self) -> usize {
        let mut n = 0;
        for i in self.path.split('/') {
            if i.len() > 0 {
                n += 1;
            }
        }
        n
    }

    pub fn as_str(&self) -> String {
        self.path.clone()
    }
}
    
fn r_first(src: &str) -> String {
    let mut c = 0;
    let mut s = String::new();
    for i in src.chars() {
        if !(c == 0 && i == '/') {
            s.push(i);
        }
        c += 1;
    }
    s
}

