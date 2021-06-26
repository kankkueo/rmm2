use std::fs;
use std::io;
//use xmltree::Element;

pub mod xml;
use crate::files;
use crate::paths::Path;

struct FomodFile {
    source: Path,
    destination: Path,
    ftype: String,
}

struct FomodPlugin {
    name: String,
    image: String,
    description: String,
    files: Vec<FomodFile>,
    type_desc: String,
}

struct FomodGroup {
    name: String,
    gtype: String,
    plugins: Vec<FomodPlugin>,
}

impl FomodPlugin {
    fn new() -> FomodPlugin {
        FomodPlugin {
            name: String::new(),
            image: String::new(),
            description: String::new(),
            files: Vec::new(),
            type_desc: String::new(),
        }
    }
}

impl FomodGroup {
    fn new() -> FomodGroup {
       FomodGroup {
            name: String::new(),
            gtype: String::new(),
            plugins: Vec::new(),
        }
    }
}


fn fix_case(src: &str) -> String {
    let mut dest = String::new();
    let mut k = 0;
    for i in src.chars() {
        if  k == 0 {
            dest.push(i.to_ascii_uppercase());
        }
        else {
            dest.push(i.to_ascii_lowercase());
        }
        k += 1;
    }
    dest
}

fn fix_case_path(src: Path) -> Path {
    let mut dest = String::new();
    for i in src.items().iter() {
        dest.push('/');
        dest.push_str(&fix_case(i));
    }
    Path::from(&dest)
}

pub fn cap_dir(src: &Path) {
    let contents: Vec<String> = files::read_datadir(src);
    for i in 0..contents.len() {

        let dir = src.clone().push(&contents[i]);
        let dir_c = src.clone().push(&fix_case(&contents[i]));

        if dir.is_dir() {
            fs::rename(dir.as_str(), dir_c.as_str()).unwrap();
            cap_dir(&dir_c);
        }
    }
}

fn unpack(src: &Path, dest: &Path) -> io::Result<()> {

    let file = fs::File::open(src.as_str())?;
    let mut archive = zip::ZipArchive::new(&file)?;

    //testi
    let full_size: f32 = file.metadata()?.len() as f32;
    let mut file_size: f32 = 0.0;

    for i in 0..archive.len() {
        let mut path = dest.as_str().clone();
        let mut file = archive.by_index(i)?;        
            match file.enclosed_name() {
                Some(pth) => pth.to_owned(),
                None => continue,
            };

            path.push_str(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&path)?;
            }
            else {
                let mut outfile = fs::File::create(&path)?;
                io::copy(&mut file,&mut outfile)?;
                file_size += outfile.metadata()?.len() as f32;
            }
            println!("{}%", (file_size/(full_size*2.0))*100.0);
        };
    println!("100.0%");
    Ok(())
}

fn check_if_fomod(src: &Path) -> bool {
    let contents = files::read_datadir(src);
    for i in contents.iter() {
        if i.contains("Fomod") || i.contains("fomod") {
            let contents = files::read_datadir(&src.clone().push(i));
            for k in contents.iter() {
                if k.contains("ModuleConfig") {
                    return true;
                }
            }
        }   
    }
    false
}

fn read_fomod_plugin(element: xmltree::Element) -> FomodPlugin {
    let mut plugin = FomodPlugin::new();
    plugin.name = element.attributes["name"].clone();
    let traits = xml::get_children_all(element);
    
    for i in 0..traits.len() {
        if traits[i].name == "description" {
            match traits[i].get_text() {
                Some(x) => {
                    plugin.description = x.to_string();
                },
                None => continue,
            }
        }
        else if traits[i].name == "image" {
            plugin.image = traits[i].attributes["path"].clone();
        }
        else if traits[i].name == "typeDescriptor" {
            match traits[i].get_child("type") {
                Some(x) => {
                    plugin.type_desc = x.attributes["name"].clone();
                }
                None => continue,
            }
        }
        else if traits[i].name == "files" {
            let files_e = xml::get_children_all(traits[i].clone());
            let mut files: Vec<FomodFile> = Vec::new();

            for j in 0..files_e.len() {
                files.push(
                    FomodFile {
                        source: Path::from(&files_e[j].attributes["source"].clone()),
                        destination: fix_case_path(Path::from(&files_e[j].attributes["destination"].clone())),
                        ftype: files_e[j].name.clone(),
                    }
                );
            }
            plugin.files = files;
        }
    }
    plugin
}

fn read_install_step(element: xmltree::Element) -> Vec<FomodGroup> {
    let groups = xml::get_children_r(element, "group");
    let mut groups_v: Vec<FomodGroup> = Vec::new();

    for i in 0..groups.len() {

        let plugins = xml::get_children_r(groups[i].clone(), "plugin");
        let mut plugins_v: Vec<FomodPlugin> = Vec::new();

        for j in 0..plugins.len() {
            plugins_v.push(read_fomod_plugin(plugins[j].clone()));
        }

        groups_v.push(
            FomodGroup {
                name: groups[i].attributes["name"].clone(),
                gtype: groups[i].attributes["type"].clone(),
                plugins: plugins_v,
            }
        );
    }
    groups_v
}

fn move_files_all(src: &Path, dest: &Path) -> io::Result<()> {
    let contents: Vec<String> = files::read_datadir(src);
    for i in 0..contents.len() {

        let src_p = src.clone().push(&contents[i]);
        let dest_p = dest.clone().push(&contents[i]);

        if src_p.is_dir() {
            fs::create_dir_all(&dest_p.as_str())?;
            move_files_all(&src_p, &dest_p)?;
        }
        else {
            fs::rename(src_p.as_str(), dest_p.as_str())?;
        }
    }
    Ok(())
}

fn install_fomod_files(plugin: &FomodPlugin, src: &Path, dest: &Path) -> io::Result<()> {
    for i in 0..plugin.files.len() {
        let src_p = src.clone().push_p(plugin.files[i].source.clone());
        let dest_p = dest.clone().push_p(plugin.files[i].destination.clone());

        if plugin.files[i].ftype == "file" {
            if !dest_p.is_dir() {
                fs::create_dir_all(dest_p.previous().as_str())?;
            }
            fs::rename(src_p.as_str(), dest_p.as_str())?;
        }
        else if plugin.files[i].ftype == "folder" {
            move_files_all(&src_p, &dest_p)?;
        }
    }
    Ok(())
}

fn selection() -> Vec<usize> {
    let mut selected: Vec<usize> = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    for i in input.split_whitespace() {
        selected.push(i.parse::<usize>().unwrap() - 1);
    }
    selected
}

fn print_plugins(group: &FomodGroup) {
    for i in 0..group.plugins.len() {
        println!("{}) {}", i + 1, group.plugins[i].name);
    }
}

fn install_fomod(src: Path, dest: Path) -> io::Result<()> {
    let src_p = src.clone().next();
    let installtxt = src.clone().push("Fomod/ModuleConfig.xml");

    let i_steps = xml::get_children_r(xml::read_xml_file(installtxt.as_str()), "installStep");

    for i in 0..i_steps.len() {
        let groups = read_install_step(i_steps[i].clone());
        for j in 0..groups.len() {
            if groups[j].gtype == "SelectExactlyOne" || groups[j].gtype == "selectexactlyone" {
                println!("Select one");
                print_plugins(&groups[j]);
                let sclt = selection()[0];
                install_fomod_files(&groups[j].plugins[sclt], &src_p, &dest)?;
            }
            else if groups[j].gtype == "SelectAny" || groups[j].gtype == "selectany" {
                println!("Select any");
                print_plugins(&groups[j]);
                let sclt = selection();

                for k in 0..sclt.len() {
                    install_fomod_files(&groups[j].plugins[sclt[k]], &src_p, &dest)?;
                }

            }
            else if groups[j].gtype == "SelectAtLeastOne" || groups[j].gtype == "selectatleastone" {
                println!("Select at least one");
                print_plugins(&groups[j]);
                let sclt = selection();

                for k in 0..sclt.len() {
                    install_fomod_files(&groups[j].plugins[sclt[k]], &src_p, &dest)?;
                }

            }
            else {
                for k in 0..groups[j].plugins.len() {
                    install_fomod_files(&groups[j].plugins[k], &src_p, &dest)?;
                }
            }
        }
    }

    fs::remove_dir_all(src_p.as_str())?;
    Ok(())

}

fn install_non_fomod(src: Path, dest: Path) -> io::Result<()> {
    let src = src.clone().next();
    cap_dir(&src);
    move_files_all(&src, &dest)?;
    Ok(())
}

pub fn install_mod(src: Path, dest: Path) -> io::Result<()> {
    let mut src_p = src.clone();

    if !src.is_dir() {
        src_p = src.previous().push("temp/");
        match unpack(&src, &src_p) {
            Err(_e) => {
                println!("Error extracting. Please extract manually and use the installer.");
                return Err(_e);
            }
            Ok(_x) => println!("Installing"),
        }
    }

    println!("{}",src_p.as_str());
    if check_if_fomod(&src_p) {
        match install_fomod(src_p, dest) {
            Err(_e) => {
                println!("Installation failed. Please install manually");
                return Err(_e);
            }
            Ok(_x) => println!("Installed successfully"),
        }
    }
    else {
        match install_non_fomod(src_p, dest) {
            Err(_e) => {
                println!("Installation failed. Please install manually");
                return Err(_e);
            }
            Ok(_x) => println!("Installed successfully"),
        }       
    }
    Ok(())
}



