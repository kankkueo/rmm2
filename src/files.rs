use std::fs;
use crate::loadorder::Plugin;
use crate::paths::Path;
use std::io;


pub fn read_datadir(path: &Path) -> io::Result<Vec<String>> {
    let mut data = String::new();
    let mut data_v: Vec<String> = Vec::new();
    let dir = fs::read_dir(path.as_str())?;
    for i in dir {
        let i = i.unwrap();
        data.push_str(&format!("{:?}",i.file_name()));
    }
    for i in data.split('"') {
        if i != "\n" && i != "" {
            data_v.push(String::from(i));
        }
    }
    Ok(data_v)
}

fn get_installed_mods(path: &Path) -> Vec<String> {
    let mut plugins: Vec<String> = Vec::new();
    let data: Vec<String> = read_datadir(path).unwrap();

    for i in data.iter() {
        if i.contains(".esp") || i.contains(".esm") {
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

pub fn get_active_mods(path_d: &Path, path_p: &Path) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = Vec::new();
    let buffer = fs::read_to_string(path_p.as_str()).expect("Could not read file");

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

pub fn write_loadorder(plugins: Vec<Plugin>, path: &Path, mode: usize) {
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
    fs::write(path.as_str(), buffer).unwrap();
}

