use std::fs;
use std::io;
//use xmltree::Element;

pub mod xml;
use crate::files;

struct FomodFile {
    source: String,
    destination: String,
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

fn get_dir(src: &str) -> String {
    let mut dir = String::new();
    let mut dir_t: Vec<&str> = Vec::new();
    for i in src.split('/') {
        dir_t.push(i);
    }
    for i in 0..dir_t.len()-1 {
        dir.push_str(dir_t[i]);
        dir.push('/');
    }
    dir
}

fn check_if_dir(path: &str) -> bool {
    match fs::read_dir(path) {
        Err(_e) => false,
        Ok(_x) => true,
    }
}

fn step_dir(path: &str) -> String {
    let contents = files::read_datadir(path);
    if contents.len() == 1 {
        let new = format!("{}/{}/", path, contents[0]);
        if check_if_dir(&new) {
            return new;
        }
    }
    path.to_string()
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

fn fix_case_path(src: &str) -> String {
    let mut dest = String::new();
    for i in src.split('/') {
        dest.push_str(&fix_case(i));
        dest.push('/');
    }
    dest.pop();
    dest
}

pub fn cap_dir(src: &str) {
    let contents: Vec<String> = files::read_datadir(src);
    for i in 0..contents.len() {

        let mut dir = format!("{}/{}/", src, contents[i]);
        let mut dir_c = format!("{}/{}/", src, fix_case(&contents[i]));

        if src.ends_with('/') {
            dir = format!("{}{}/", src, contents[i]);
            dir_c = format!("{}{}/", src, fix_case(&contents[i]));
        }

        if check_if_dir(&dir) {
            fs::rename(&dir, &dir_c).unwrap();
            cap_dir(&dir_c);
        }
    }
}

fn unpack(src: &str, dest: &str) -> io::Result<()> {

    let fname = std::path::Path::new(src);
    let file = fs::File::open(fname)?;
    let mut archive = zip::ZipArchive::new(&file)?;

    //testi
    let full_size: f32 = file.metadata()?.len() as f32;
    let mut file_size: f32 = 0.0;

    for i in 0..archive.len() {
        let mut path = String::from(dest);
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

fn check_if_fomod(src: &str) -> bool {
    let fname = std::path::Path::new(src);
    let file = fs::File::open(fname).unwrap();
    let mut archive = zip::ZipArchive::new(&file).unwrap();

    for i in 0..archive.len() {
        let file = archive.by_index(i).unwrap();        
        if (file.name().contains("Fomod") || file.name().contains("fomod")) && file.name().contains("ModuleConfig") {
            return true;
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
                        source: xml::turn_slashes(files_e[j].attributes["source"].clone()),
                        destination: xml::turn_slashes(files_e[j].attributes["destination"].clone()),
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

fn move_files_all(src: &str, dest: &str) -> io::Result<()> {
    let contents: Vec<String> = files::read_datadir(src);
    for i in 0..contents.len() {

        let mut src_p = format!("{}/{}", src, contents[i]);
        let mut dest_p = format!("{}/{}", dest, contents[i]);

        if src.ends_with('/') {
            src_p = format!("{}{}", src, contents[i]);
            dest_p = format!("{}{}", dest, contents[i]);
        }

        if check_if_dir(&src_p) {
            fs::create_dir_all(&dest_p)?;
            move_files_all(&src_p, &dest_p)?;
        }
        else {
            fs::rename(src_p, dest_p)?;
        }
    }
    Ok(())
}

fn install_fomod_files(plugin: &FomodPlugin, src: &str, dest: &str) -> io::Result<()> {
    for i in 0..plugin.files.len() {
        let src_p = format!("{}{}", src, plugin.files[i].source);
        let dest_p = format!("{}{}", dest, fix_case_path(&plugin.files[i].destination));

        if plugin.files[i].ftype == "file" {
            if !check_if_dir(&dest_p) {
                fs::create_dir_all(get_dir(&dest_p))?;
            }
            fs::rename(&src_p, &dest_p)?;
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

fn install_fomod(src: &str, dest: &str) -> io::Result<()> {
    let src_p = format!("{}{}", get_dir(src), "temp/");
    unpack(src, &src_p)?;
    let src_p = step_dir(&src_p);
    let installtxt = format!("{}{}", src_p, "Fomod/ModuleConfig.xml");

    let i_steps = xml::get_children_r(xml::read_xml_file(&installtxt), "installStep");

    for i in 0..i_steps.len() {
        let groups = read_install_step(i_steps[i].clone());
        for j in 0..groups.len() {
            if groups[j].gtype == "SelectExactlyOne" || groups[j].gtype == "selectexactlyone" {
                println!("Select one");
                print_plugins(&groups[j]);
                let sclt = selection()[0];
                install_fomod_files(&groups[j].plugins[sclt], &src_p, dest)?;
            }
            else if groups[j].gtype == "SelectAny" || groups[j].gtype == "selectany" {
                println!("Select any");
                print_plugins(&groups[j]);
                let sclt = selection();

                for k in 0..sclt.len() {
                    install_fomod_files(&groups[j].plugins[sclt[k]], &src_p, dest)?;
                }

            }
            else if groups[j].gtype == "SelectAtLeastOne" || groups[j].gtype == "selectatleastone" {
                println!("Select at least one");
                print_plugins(&groups[j]);
                let sclt = selection();

                for k in 0..sclt.len() {
                    install_fomod_files(&groups[j].plugins[sclt[k]], &src_p, dest)?;
                }

            }
            else {
                for k in 0..groups[j].plugins.len() {
                    install_fomod_files(&groups[j].plugins[k], &src_p, dest)?;
                }
            }
        }
    }

    fs::remove_dir_all(src_p)?;
    Ok(())

}

fn install_non_fomod(src: &str, dest: &str) -> io::Result<()> {
    let src_p = format!("{}{}", get_dir(src), "temp/");
    unpack(src, &src_p)?;
    let src_p = step_dir(&src_p);
    cap_dir(&src_p);
    move_files_all(&src_p, dest)?;
    fs::remove_dir_all(src_p)?;
    Ok(())
}

pub fn install_mod(src: &str, dest: &str) -> io::Result<()> {
    if check_if_fomod(src) {
        install_fomod(src, dest)?;
    }
    else {
        install_non_fomod(src, dest)?;
    }
    Ok(())
}




