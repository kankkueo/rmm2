//mod mods;
mod modinstall;
mod ui;
mod loadorder;
mod config;
mod files;
mod paths;
use std::io;


fn main() -> io::Result<()> {
    
    let mode = ui::mode_selection_menu().unwrap();
    
    let config = config::read_config(mode);
    let mut plugins = files::get_active_mods(&config.data, &config.plugins, mode);
    let mut mods = files::read_datadir(&config.mods).unwrap();

    match ui::plugin_menu(&mut plugins, &mut mods, config.clone(), mode).unwrap() {
        Some(x) => modinstall::install_mod(x, config.data),
        None => Ok(()),
    }

//    println!("{}", ui::fileexplorer().unwrap().as_str());

}

