use viuer::{Config, print_from_file};

fn config(x: u16, y: u16, w: u16, h: u16) -> Config {
    Config {
        x: x,
        y: y as i16,
        width: Some(w as u32),
        height: Some(h as u32),
        ..Default::default()
    }
}

pub fn print_image(src: &str, x: u16, y: u16, w: u16, h: u16) {
    match print_from_file(src, &config(x, y, w, h)) {
        Ok(_e) => {},
        Err(_e) => {},
    }
}
