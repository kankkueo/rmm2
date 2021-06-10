use std::fs;
use crate::loadorder::{Plugin, Fomod};
use crate::modinstall::xml;


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

pub fn get_active_mods(path: &str, path_p: &str) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = get_installed_mods(path);
    let buffer = fs::read_to_string(path_p).expect("Could not read file");
    for i in buffer.split('\n') {
        for k in 0..plugins.len() {
            if i == plugins[k].name {
                plugins[k].active = true;
            }
        }
    }
    plugins
}

pub fn write_loadorder(plugins: Vec<Fomod>, path: &str) {
    let mut buffer = String::new();
    for i in 0..plugins.len() {
        if plugins[i].plugin.active {
            buffer.push_str(&plugins[i].plugin.name);
            buffer.push('\n');
        }
    }
    fs::write(path, buffer).unwrap();
}

/*
pub fn read_modinfo(path: &str) -> Fomod {
    let mut plugins: Vec<Fomod> = Vec::new();
    let kakka = xml::read_xml_file(path);


    match kakka.get_child("Name") {
        Some(x) => match x.get_text() {
            Some(y) => { 
                let plug = Plugin {
                    name: y.to_string(),
                    active: false,
                };
            }
            None => { let plu = ""; }
        }
        None => { let name = ""; }
    }
    match kakka.get_child("Author") {
        Some(x) => match x.get_text() {
            Some(y) => { let auth = y; }
            None => { let auth = ""; }
        }
        None => { let auth = ""; }
    }
    match kakka.get_child("Groups") {
        Some(x) => match xml::get_children_all(x.clone())[0].get_text() {
            Some(y) => { let grp = y; }
            None => { let grp = ""; }
        }
        None => { let desc = ""; }
    }
    match kakka.get_child("Website") {
        Some(x) => match x.get_text() {
            Some(y) => { let webs = y; }
            None => { let webs = ""; }
        }
        None => { let webs = ""; }
    }
    match kakka.get_child("Version") {
        Some(x) => match x.get_text() {
            Some(y) => { let vers = y; }
            None => { let vers = ""; }
        }
        None => { let vers = ""; }
    }
 
}
*/

