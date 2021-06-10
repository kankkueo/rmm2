//mod mods;
mod modinstall;
mod ui;
mod loadorder;
use std::io;


fn main() -> io::Result<()> {
    let config = modinstall::config::read_config(1);
    let mut plugins = modinstall::files::get_active_mods(&config.data, &config.plugins, 1);
    ui::plugin_menu(&mut plugins, &config.plugins, 1).unwrap();

    Ok(()) 
}

