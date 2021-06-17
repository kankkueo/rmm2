use std::fs;
use crate::loadorder::Plugin;


pub fn read_datadir(path: &str) -> Vec<String> {
    let mut data = String::new();
    let mut data_v: Vec<String> = Vec::new();
    let dir = fs::read_dir(path).expect("Could not read");
    for i in dir {
        let i = i.unwrap();
        data.push_str(&format!("{:?}",i.file_name()));
    }
    for i in data.split('"') {
        if i != "\n" && i != "" {
            data_v.push(String::from(i));
        }
    }
    data_v
}

fn get_installed_mods(path: &str) -> Vec<String> {
    let mut plugins: Vec<String> = Vec::new();
    let data: Vec<String> = read_datadir(path);

    for i in data.iter() {
        if i.contains(".esp") {
            plugins.push(i.to_string());
        }
    }
    plugins
}

fn ignore_asterix(src: &str) -> String {
    let mut buf = String::new();
    for i in src.chars() {
        if i != '*' {
            buf.push(i);
        }
    }
    buf 
}

pub fn get_active_mods(path_d: &str, path_p: &str, mode: usize) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = Vec::new();
    let buffer = fs::read_to_string(path_p).expect("Could not read file");

    for i in buffer.split('\n') {
        if i != "" && i != " " && i != "\n" {
            plugins.push( Plugin {
                name: ignore_asterix(i),
                active: true,
            } );
        }
    }

    let installed = get_installed_mods(path_d);

    for i in installed.iter() {
        let mut act = false;
        for j in 0..plugins.len() {
            if i == &plugins[j].name {
                act = true;
            }
        }
        if !act {
            plugins.push( Plugin {
                name: i.to_string(),
                active: false,
            });
        }
    }
    plugins
}

pub fn write_loadorder(plugins: Vec<Plugin>, path: &str, mode: usize) {
    let mut buffer = String::new();
    for i in 0..plugins.len() {
        if mode == 1 || mode == 3 {
            if plugins[i].active {
                buffer.push('*');
                buffer.push_str(&plugins[i].name);
                buffer.push('\n');
            }
        }
        else {
            if plugins[i].active {
                buffer.push_str(&plugins[i].name);
                buffer.push('\n');
            }
        }
    }
    fs::write(path, buffer).unwrap();
}

