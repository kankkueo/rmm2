use std::fs;
//use std::path::Path;
use crate::loadorder::Plugin;
use crate::paths::Path;
use crate::config::Mode;
use std::io;

// read aĺl files in a directory
pub fn read_directory(path: &Path) -> io::Result<Vec<String>> {
    let mut entries: Vec<String> = Vec::new();

    for entry in fs::read_dir(path.as_str())? {
        let filename = entry?.file_name().to_str().unwrap().to_string();
        entries.push(filename);
    }

    Ok(entries)
}

// recursively goes through mod directories and finds all plugin files
pub fn find_plugins(path: &Path, plugins: &mut Vec<String>) -> io::Result<()> {

    for entry in fs::read_dir(path.as_str())? {
        let entry = entry?;
        if entry.path().is_dir() {
            // remove once std:path:Path in implemented
            let paath = Path::from(entry.path().to_str().unwrap());
            find_plugins(&paath, plugins)?;
        }
        else {
            let filename = entry.file_name().to_str().unwrap().to_string();
            if filename.contains(".esp") || filename.contains(".esm") {
                plugins.push(filename);
            }
        }
    }
    Ok(())
}

/*  OBSOLETE
fn read_plugins_dir(path: &Path) -> Vec<String> {
    let mut plugins: Vec<String> = Vec::new();
    let data: Vec<String> = read_directory(path).unwrap();

    for i in data.iter() {
        if i.contains(".esp") || i.contains(".esm") {
            plugins.push(i.to_string());
        }
    }
    plugins
}
*/

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
    let mut installed: Vec<String> = Vec::new();
    find_plugins(path_d, &mut installed);

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

pub fn write_loadorder(plugins: Vec<Plugin>, path: &Path, mode: Mode) {
    let mut buffer = String::new();
    let mut prefix = "";

    match mode {
        Mode::Fallout4 | Mode::SkyrimSE => {
            prefix = "*";
        }
        _default => {}
    }

    for i in 0..plugins.len() {
        if plugins[i].active {
            buffer.push_str(prefix);
            buffer.push_str(&plugins[i].name);
            buffer.push('\n');
        }
    }
    fs::write(path.as_str(), buffer).unwrap();
}

