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
    ui::plugin_menu(&mut plugins, &mut mods, config.clone(), mode, &events).unwrap();


    Ok(()) 
}

