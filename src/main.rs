//mod mods;
mod modinstall;
mod ui;
mod loadorder;
use std::io;

const _PATH: &str = "/home/elmo/koodi/rmm/testmod/Fomod/info.xml";
const _DPATH: &str = "/home/elmo/koodi/rmm/testmod2";
const _ZIP: &str = "/home/elmo/koodi/rmm/testmod3/Climates Of Tamriel Special Edition-2237-5-7.zip"; 
const _ZIPD: &str = "/home/elmo/koodi/rmm/testmod3/unpacked/";
const _DIRRI: &str = "/home/elmo/koodi/rmm/testmod3/fdsaf/";


fn main() -> io::Result<()> {
    modinstall::configtest(3);

    Ok(()) 
}

