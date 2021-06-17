//mod mods;
mod modinstall;
mod ui;
mod loadorder;
mod config;
mod files;
use std::io;


fn main() -> io::Result<()> {
    let config = config::read_config(1);
    let mut plugins = files::get_active_mods(&config.data, &config.plugins, 1);
    let mut mods = files::read_datadir(&config.mods);
    ui::plugin_menu(&mut plugins, &mut mods, config.clone(), 1).unwrap();


    Ok(()) 
}

