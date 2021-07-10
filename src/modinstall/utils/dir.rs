use std::{io, fs};
use crate::paths::Path;
use crate::files::read_datadir;
use crate::ui::utils::keyin;

pub fn fix_case(src: &str) -> String {
    let mut dest = String::new();
    for i in src.chars() {
        dest.push(i.to_ascii_lowercase());
    }
    dest
}

pub fn cap_dir(src: &Path) -> io::Result<()> {
    let contents: Vec<String> = read_datadir(src)?;
    for i in 0..contents.len() {

        let dir = src.clone().push(&contents[i]);
        let dir_c = src.clone().push(&fix_case(&contents[i]));

        if dir.is_dir() {
            fs::rename(dir.as_str(), dir_c.as_str())?;
            cap_dir(&dir_c)?;
        }
    }
    Ok(())
}

pub fn cap_dir_all(src: &Path) -> io::Result<()> {
    let contents: Vec<String> = read_datadir(src)?;
    for i in 0..contents.len() {

        let dir = src.clone().push(&contents[i]);
        let dir_c = src.clone().push(&fix_case(&contents[i]));

        fs::rename(dir.as_str(), dir_c.as_str())?;
        if dir_c.is_dir() {
            cap_dir_all(&dir_c)?;
        }
    }
    Ok(())
}

fn dir_r(src: &Path, v: &mut Vec<String>) -> io::Result<()> {
    let contents = read_datadir(src)?;
    for i in contents.iter() {
        let d = src.clone().push(i);

        if d.is_dir() {
            dir_r(&d, v)?;
        } 
        else {
            v.push(d.as_str());
        }
    }
    Ok(())
}

pub fn read_dir_r(src: &Path) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    match dir_r(src, &mut paths) {
        _default => paths
    }
}

pub fn check_if_fomod(src: &Path) -> bool {
    let contents = read_dir_r(src);
    for i in contents.iter() {
        if fix_case(i).contains("fomod") && fix_case(i).contains("moduleconfig") {
            return true;
        }
    }
    false
}

pub fn find_installfile(src: &Path) -> Path {
    let d = read_dir_r(&src);
    for i in d.iter() {
        let p = Path::from(i);
        if fix_case(&p.as_str()).ends_with("fomod/moduleconfig.xml") {
            return p;
        }
    }
    return Path::new();
}

pub fn mod_root(src: &Path) -> Path {
    let items = vec![
        "textures",
        "meshes",
        "interface",
        "sound",
        "scripts",
    ];

    let mut p = src.clone();
    while read_datadir(&p).unwrap().len() < 2 {
        p.next();
        for i in items.iter() {
            if fix_case(&p.as_str()).contains(i) { return p; }
        }
    }
    p
}

pub fn move_files_all(src: &Path, dest: &Path) -> io::Result<()> {
    let contents: Vec<String> = read_datadir(src).unwrap();
    for i in 0..contents.len() {

        let src_p = src.clone().push(&contents[i]);
        let dest_p = dest.clone().push(&contents[i]);

        println!("{}\n{}", src_p.as_str(), dest_p.as_str());

        if src_p.is_dir() {
            fs::create_dir_all(&dest_p.as_str())?;
            move_files_all(&src_p, &dest_p)?;
        }
        else {
            match fs::rename(src_p.as_str(), dest_p.as_str()) {
                Ok(_x) => {},
                Err(_e) => { println!("File not found!\nPress enter to ignore"); keyin();}
            }
        }
    }
    Ok(())
}
