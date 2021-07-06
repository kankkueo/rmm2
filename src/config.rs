use std::fs;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io;
use crate::modinstall::utils::dir::{cap_dir, fix_case};
use crate::paths::Path;
use crate::ui::fileexplorer;

#[derive(Deserialize, Serialize)]
struct GamepathsT {
    skyrimse: Option<GamepathT>,     //1
    skyrim: Option<GamepathT>,       //2
    oblivion: Option<GamepathT>,     //3
    fallout4: Option<GamepathT>,     //4
    falloutnv: Option<GamepathT>,    //5
    fallout3: Option<GamepathT>,     //6
}

#[derive(Deserialize, Serialize, Clone)]
struct GamepathT {
    data: String,
    plugins: String,
    mods: String,
}

#[derive(Clone)]
pub struct Gamepath {
    pub data: Path,
    pub plugins: Path,
    pub mods: Path,
}

impl GamepathT {
    fn to_gp(&self) -> Gamepath {
        Gamepath {
            data: Path::from(&self.data),
            plugins: Path::from(&self.plugins),
            mods: Path::from(&self.mods),
        }
    }
}

impl Gamepath {
    fn to_gpt(&self) -> GamepathT {
        GamepathT {
            data: self.data.as_str().to_string(),
            plugins: self.plugins.as_str().to_string(),
            mods: self.mods.as_str().to_string(),
        }
    }
}

fn create_conf_file(conf: &Path) -> io::Result<()> {
//    let dir = format!("{}{}", env::var("HOME").unwrap(), "/config/rmm2/");
    let dir = conf.clone().previous(); 
    fs::create_dir_all(dir.as_str())?;
    fs::write(conf.as_str(), "")
}

fn write_conf_file(config: &GamepathsT) -> io::Result<()> {
    let conf = format!("{}{}",env::var("HOME").unwrap(), "/.config/rmm2/config");
    //let content = format!("[Gamepaths]\n\n{}", toml::to_string(&config).unwrap());
    let content = toml::to_string(config).unwrap();
    fs::write(conf, content)
}

fn create_game_conf(mode: usize) -> Gamepath {
    let mut p = fileexplorer("Navigate to your game's data directory").unwrap();
    while !check_path(&p) { 
        println!("Invalid path!");
        p = fileexplorer("Navigate to your game's data directory").unwrap();
    }

    match cap_dir(&p) {
        Ok(_x) => {},
        Err(_e) => {
            println!("Invalid path!");
            create_game_conf(mode);
        }
    }

    Gamepath {
        data: p.clone(),
        plugins: get_plugin_path(&p, mode),
        mods: get_mod_path(&p),
    }
}

fn check_path(path: &Path) -> bool {
    for i in path.items() {
        if fix_case(&i).contains("data") {
            return true;
        }
    }
    false
}

fn get_mod_path(path: &Path) -> Path {
    let modsp = path.previous().push("mods");
    if !modsp.is_dir() {
        fs::create_dir_all(modsp.as_str()).unwrap();
    }
    modsp
}

fn create_plugin_file(path: &Path) -> io::Result<()> {
    match fs::read_to_string(path.as_str()) {
        Ok(_x) => Ok(()),
        Err(_e) => fs::write(path.as_str(), ""),
    }
}

fn get_plugin_path(path: &Path, mode: usize) -> Path {
    let mut buff = Path::new();
    let paths = vec![
        "/steamapps/compatdata/489830/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim Special Edition/plugins.txt",
        "/steamapps/compatdata/72850/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim/plugins.txt",
        "/steamapps/compatdata/22330/pfx/drive_c/users/steamuser/Local Settings/Application Data/Oblivion/plugins.txt",
        "/steamapps/compatdata/377160/pfx/drive_c/users/steamuser/Local Settings/Application Data/Fallout4/plugins.txt",
        "/steamapps/compatdata/22380/pfx/drive_c/users/steamuser/Local Settings/Application Data/Fallout New Vegas/plugins.txt",
        "/steamapps/compatdata/22370/pfx/drive_c/users/steamuser/My Documents/My Games/Fallout3/plugins.txt",
    ];

    for i in path.items() {
        if i == "steamapps" {
            buff.push(paths[mode - 1]);
            create_plugin_file(&buff).unwrap();
            return buff;
        }
        else if i.starts_with('$') {
            buff.push(&envvar(i));
        }
        else {
            buff.push(&i);
        }
    }

    let buf2 = fileexplorer("Could not find plugins file. Please enter the path manually").unwrap();

    match create_plugin_file(&buf2) {
        Ok(()) => buf2,
        Err(_e) => {
            println!("Please enter a valid path");
            get_plugin_path(path, mode)
        }
    }
}

fn read_toml(paths: GamepathsT, mode: usize) -> Gamepath {
    let mut paths = paths;
    match mode {
        1 => match paths.skyrimse {
            Some(x) => x.to_gp(),
            None => {
                paths.skyrimse = Some(create_game_conf(1).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        2 => match paths.skyrim {
            Some(x) => x.to_gp(),
            None => {
                paths.skyrim = Some(create_game_conf(2).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        3 => match paths.oblivion {
            Some(x) => x.to_gp(),
            None => {
                paths.oblivion = Some(create_game_conf(3).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        4 => match paths.fallout4 {
            Some(x) => x.to_gp(),
            None => {
                paths.fallout4 = Some(create_game_conf(4).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        5 => match paths.falloutnv {
            Some(x) => x.to_gp(),
            None => {
                paths.falloutnv = Some(create_game_conf(5).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        6 => match paths.fallout3 {
            Some(x) => x.to_gp(),
            None => {
                paths.fallout3 = Some(create_game_conf(6).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        _default => panic!("Invalid mode!"),
    }
}

pub fn read_config(mode: usize) -> Gamepath {
    let conf = Path::from(&env::var("HOME").unwrap()).push(".config/rmm2/config");

    match fs::read_to_string(&conf.as_str()) {
        Ok(x) => read_toml(toml::from_str(&x).unwrap(), mode), 
        _default => {
            create_conf_file(&conf).unwrap(); 
            read_config(mode)
        }
    }
}

fn envvar(src: String) -> String {
    let mut v = String::new();
    for i in src.chars() {
        if i != '$' {
            v.push(i);
        }
    }
    match env::var(v) {
        Ok(x) => x,
        Err(_e) => src,
    }
}

