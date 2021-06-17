//mod mods;
mod modinstall;
mod ui;
mod loadorder;
mod config;
mod files;
use std::io;


fn main() -> io::Result<()> {
    let events = ui::events::Events::new();
    let mode = ui::mode_selection_menu(&events).unwrap();
    let config = config::read_config(mode);
    let mut plugins = files::get_active_mods(&config.data, &config.plugins, mode);
    let mut mods = files::read_datadir(&config.mods);
    match ui::plugin_menu(&mut plugins, &mut mods, config.clone(), mode, &events).unwrap() {
        Some(x) => match modinstall::install_mod(&x, &config.data) {
            Ok(_y) => println!("Done"),
            _default => println!("Error installing mod. Please install manually"),
        }
        None => println!("exiting"),
    }


    Ok(()) 
}

