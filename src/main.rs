mod modinstall;
mod ui;
mod loadorder;
mod config;
mod files;
mod paths;
use std::io;


fn main() -> io::Result<()> {
    
    //let mode = ui::mode_selection_menu().unwrap();
    //let mode = 1;
    
    //let config = config::read_config(mode);
    //let mut plugins = files::get_active_mods(&config.data, &config.plugins);
    //let mut mods = files::read_datadir(&config.mods).unwrap();

    let mut test: Vec<String> = Vec::new();
    files::find_plugins(&paths::Path::from("/data/storage/SteamLX/steamapps/common/Skyrim Special Edition/"), &mut test)?;

    for i in test {
        println!("{}", i);
    }

   /* 
    loop {

        match ui::plugin_menu(&mut plugins, &mut mods, config.clone(), mode).unwrap() {
            Some(x) => match modinstall::install_mod(x, config.data.clone()) {
                Ok(_x) => {
                    println!("Install successful!\n Press enter to continue");
                    ui::utils::keyin();
                    mods = files::read_datadir(&config.mods).unwrap();
                    plugins = files::get_active_mods(&config.data, &config.plugins);
                },
                Err(e) => panic!("Install failed: {}", e),
            }
            None => break,
        }
    }
    */

    Ok(())

}

