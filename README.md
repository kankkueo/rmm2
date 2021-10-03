# rmm2
A simple mod manager for The Elder Scrolls and Fallout games on Linux (proton). Allows installing mods and loadorder management. Supports Fomod installers.

## Building from source
#### 1. install rust/cargo

On Debian based distros: `apt install cargo`

On Arch: `pacman -S rust` or `pacman -S rustup` for the toolchain

#### 2. Clone or download the repository. 

Using git: `git clone https://github.com/e-k1/rmm2.git`

Or just download and extract the archive.

#### 3. Build

While in the downloaded directory, compile with: `cargo build`

#### 4. Make it runnable from the console

`echo $PATH` to see where executable binaries are stored

Copy the binary to your path: `cp target/debug/rmm2 <your path>`

For example: `cp target/debug/rmm2 /usr/local/bin`

### Installing from releases

You can also ownload the binary from the [releases](https://github.com/e-k1/rmm2/releases) page, make it executable with `chmod +x rmm2` and copy it to your path.

## Usage

Start the program by typing rmm2 in the terminal. Choose the game you want to manage and insert the path of the data directory of that game. You must have launched the game at least once in order for the program to work. 

Place the mods you wish to install in the newly created 'mods' directory in your game's root directory and select them in the menu. Mods can be installed from zip archives or directories, though directories are preferred, as the zip feature is unstable and doesn't work most of the time. 

It is recommended to use Kitty terminal, since it allows images to be displayed at full resolution. Other terminals work, but images won't diplay properly.


## Issues

As stated, unpacking archives often fails. If this happens, extract the archive into a directory inside the mods directory and try installing again.

