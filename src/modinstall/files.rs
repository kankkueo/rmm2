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

fn get_installed_mods(path: &str) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = Vec::new();
    let data: Vec<String> = read_datadir(path);

    for i in 0..data.len() {
        if data[i].contains(".esp") {
            let plugin_t = Plugin {
                name: data[i].clone(),
                active: false,    
            };
            plugins.push(plugin_t);
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
    let mut plugins: Vec<Plugin> = get_installed_mods(path_d);
    let buffer = fs::read_to_string(path_p).expect("Could not read file");

    for i in buffer.split('\n') {
        for k in 0..plugins.len() {
            if mode == 1 || mode == 3 {
                if ignore_asterix(i) == plugins[k].name {
                    plugins[k].active = true;
                }
            }
            else {
                if i == plugins[k].name {
                    plugins[k].active = true;
                }
            }
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

