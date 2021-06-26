use std::fs;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io;
use crate::modinstall::cap_dir;
use crate::paths::Path;

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

fn create_conf_file(conf: &str) -> io::Result<()> {
    let dir = format!("{}{}", env::var("HOME").unwrap(), "/config/rmm2/");
    fs::create_dir_all(dir.clone())?;
    fs::write(conf, "")
}

fn write_conf_file(config: &GamepathsT) -> io::Result<()> {
    let conf = format!("{}{}",env::var("HOME").unwrap(), "/.config/rmm2/config");
    let content = format!("[Gamepaths]\n\n{}", toml::to_string(&config).unwrap());
    fs::write(conf, content)
}

fn create_game_conf(mode: usize) -> Gamepath {
    let mut d = String::new();
    println!("Enter the path to your game's data directory (absolute path)");
    io::stdin().read_line(&mut d).unwrap();

    Gamepath {
        data: fix_data_path(Path::from(&d)),
        plugins: get_plugin_path(Path::from(&d), mode),
        mods: get_mod_path(Path::from(&d)),
    }
}

fn fix_data_path(path: Path) -> Path {
    let mut buff = Path::new();
    for i in path.items() {
        if (i == "Data" || i == "data") && buff.as_str().contains("common") {
            buff.push("Data/");
            break;
        }
        else {
            buff.push(&i);
        }
    }
    cap_dir(&buff);
    buff
}

fn get_mod_path(path: Path) -> Path {
    let mut buff = Path::new();
    for i in path.items() {
        if (i == "Data" || i == "data") && buff.as_str().contains("common") {
            buff.push("Mods/");
            break;
        }
        else {
            buff.push(&i);
        }
        
    }
    fs::create_dir_all(&buff.as_str()).unwrap();
    buff
}

fn create_plugin_file(path: &Path) -> io::Result<()> {
    match fs::read_to_string(path.as_str()) {
        Ok(_x) => Ok(()),
        Err(_e) => fs::write(path.as_str(), ""),
    }
}

fn get_plugin_path(path: Path, mode: usize) -> Path {
    let mut buff = Path::new();
    let paths = vec![
        "/steamapps/compatdata/489830/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim Special Edition/plugins.txt",
        "/steamapps/compatdata/489830/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim Special Edition/plugins.txt",
        "/steamapps/compatdata/489830/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim Special Edition/plugins.txt",
        "/steamapps/compatdata/489830/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim Special Edition/plugins.txt",
        "/steamapps/compatdata/489830/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim Special Edition/plugins.txt",
        "/steamapps/compatdata/489830/pfx/drive_c/users/steamuser/Local Settings/Application Data/Skyrim Special Edition/plugins.txt",
    ];

    for i in path.items() {
        if i == "steamapps" {
            buff.push(paths[mode]);
            create_plugin_file(&buff).unwrap();
            return buff;
        }
        else {
            buff.push(&i);
        }
    }

    println!("Could not find plugins file. Please enter the path manually (absolute path)");

    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).unwrap();
    let buf2 = Path::from(&buf2);

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
                paths.skyrim = Some(create_game_conf(1).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        3 => match paths.oblivion {
            Some(x) => x.to_gp(),
            None => {
                paths.oblivion = Some(create_game_conf(1).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        4 => match paths.fallout4 {
            Some(x) => x.to_gp(),
            None => {
                paths.fallout4 = Some(create_game_conf(1).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        5 => match paths.falloutnv {
            Some(x) => x.to_gp(),
            None => {
                paths.falloutnv = Some(create_game_conf(1).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        6 => match paths.fallout3 {
            Some(x) => x.to_gp(),
            None => {
                paths.fallout3 = Some(create_game_conf(1).to_gpt());
                write_conf_file(&paths).unwrap();
                read_toml(paths, mode)
            }
        }
        _default => panic!("Invalid mode!"),
    }
}

pub fn read_config(mode: usize) -> Gamepath {
    //let conf = format!("{}{}",env::var("HOME").unwrap(), "/.config/rmm2/config");
    let conf = Path::from(&env::var("HOME").unwrap()).push(".config/rmm2/config");

    match fs::read_to_string(&conf.as_str()) {
        Ok(x) => read_toml(toml::from_str(&x).unwrap(), mode), 
        _default => {
            create_conf_file(&conf.as_str()).unwrap(); 
            read_config(mode)
        }
    }
}




