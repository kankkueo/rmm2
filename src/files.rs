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
        if i.len() > 1  {
            data_v.push(format_mod_name(i));
        }
    }

    data_v.sort();
    Ok(data_v)
}

fn read_plugins_dir(path: &Path) -> Vec<String> {
    let mut plugins: Vec<String> = Vec::new();
    let data: Vec<String> = read_datadir(path).unwrap();

    for i in data.iter() {
        if i.contains(".esp") || i.contains(".esm") {
            plugins.push(i.to_string());
        }
    }
    plugins
}

fn read_plugins_file(path: &Path) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = Vec::new();
    let buffer = fs::read_to_string(path.as_str()).expect("Could not read file");

    for i in buffer.split('\n') {
        if i.len() > 1 {
            plugins.push( Plugin {
                name: format_mod_name(i),
                active: true,
            } );
        }
    }
    plugins
}

pub fn get_active_mods(path_d: &Path, path_p: &Path) -> Vec<Plugin> {
    let mut plugins = read_plugins_file(path_p);
    let installed = read_plugins_dir(path_d);

    for i in installed.iter() {
        let mut act = false;
        for j in 0..plugins.len() {
            if i == &plugins[j].name {
                act = true;
            }
        }
        if !act {
            plugins.push( Plugin {
                name: format_mod_name(i),
                active: false,
            });
        }
    }
    plugins
}

fn format_mod_name(src: &str) -> String {
    let mut buf = String::new();

    if src.contains("\'") || src.starts_with('*') {
        for i in src.chars() {
            if i != '*' && i != '\\' {
                buf.push(i);
            }
        }
    }
    else { return src.to_string(); }
    buf 
}

pub fn write_loadorder(plugins: Vec<Plugin>, path: &Path, mode: usize) {
    let mut buffer = String::new();
    for i in 0..plugins.len() {
        if mode == 1 || mode == 4 {
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

