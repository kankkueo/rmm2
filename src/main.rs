//mod mods;
mod modinstall;
mod ui;
mod loadorder;
use std::io;


fn main() -> io::Result<()> {
    let config = modinstall::config::read_config(1);
    let mut plugins = modinstall::files::get_active_mods(&config.data, &config.plugins, 1);
    let mut mods = modinstall::files::read_datadir(&config.mods);
    ui::plugin_menu(&mut plugins, &mut mods, config.clone(), 1).unwrap();


    Ok(()) 
}

