use std::fs::read_dir;
use crate::files::read_datadir;

#[derive(Clone)]
pub struct Path {
    pub path: String,
    size: usize,
}

impl Path {
    pub fn from(src: &str) -> Path {
        let mut cnt = 0;
        let mut s = String::new();

        for i in src.chars() {
            if i == '/' || i == '\\' {
                s.push('/');
                cnt += 1;
            }
            else {
                s.push(i);
            }
        }

        if s.ends_with('/') { cnt -= 1; }
        
        Path {
            path: s,
            size: cnt,
        }
    }

    pub fn push(&mut self, t: &str) -> Path {
        if self.path.ends_with('/') {
            self.path.push_str(t);
        }
        else {
            self.path.push('/');
            self.path.push_str(t);
        }
        self.size += 1;
        self.clone()
    }

    pub fn is_dir(&self) -> bool {
        match read_dir(&self.path) {
            Err(_e) => false,
            Ok(_x) => true,
        }
    }

    pub fn previous(&self) -> Path {
        let mut cnt = 0;
        let mut s = String::new();

        for i in self.path.split('/') {
            if cnt == self.size-1 { break; }
            s.push('/');
            s.push_str(i);
            cnt += 1;
        }

        Path {
            path: s,
            size: cnt,
        }
    }

    pub fn next(&mut self) -> Path {
        if self.is_dir() {
            let contents = read_datadir(&self.path);
            if contents.len() == 1 {
                self.push(&contents[0]);
            }
        }
        self.clone()
    }
}

