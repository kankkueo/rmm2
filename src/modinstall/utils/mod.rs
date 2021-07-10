use std::fs;
use std::io;
use crate::paths::Path;
use crate::ui::utils::keyin;

pub mod xml;
pub mod dir;

struct FomodFile {
    source: Path,
    destination: Path,
    pub ftype: String,
}

struct FomodPlugin {
    name: String,
    image: Path,
    description: String,
    type_desc: String,
    files: Vec<FomodFile>,
    c_flags: Option<Vec<ConditionFlag>>,
}

pub struct FomodGroup {
    name: String,
    gtype: String,
    plugins: Vec<FomodPlugin>,
}

struct ConditionFlag {
    name: String,
    active: bool,
}

struct Pattern {
    deps: Vec<String>,
    oper: String,
    files: Vec<FomodFile>,
}

impl FomodFile {
    fn install(&self) -> io::Result<()> {
        if self.ftype == "file" {

            println!("{}\n{}", self.source.as_str(), self.destination.as_str());
            fs::create_dir_all(self.destination.previous().as_str())?; 

            match fs::rename(self.source.as_str(), self.destination.as_str()) {
                Ok(_x) => {},
                Err(_e) => { 
                    println!("File not found!\nPress enter to ignore");
                    keyin();
                }
            }
        }
        else if self.ftype == "folder" {
            dir::move_files_all(&self.source, &self.destination)?;
        }

        Ok(())
    }
}

impl FomodPlugin {
    fn new() -> FomodPlugin {
        FomodPlugin {
            name: String::new(),
            image: Path::new(),
            description: String::new(),
            type_desc: String::new(),
            files: Vec::new(),
            c_flags: None,
        }
    }

    fn install_files(&self) -> io::Result<()> {
        for i in self.files.iter() {
            i.install()?;
        }
        Ok(())
    }
}

impl FomodGroup {
    pub fn plugins(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new(); 
        for i in self.plugins.iter() {
            v.push(i.name.clone());
        }
        v
    }

    pub fn showinfo(&self, index: usize) -> String {
        format!("{}\n\n{}",
            self.plugins[index].type_desc,
            self.plugins[index].description,
        )
    }

    pub fn title(&self) -> String {
        let mut s = String::new();
        for i in self.gtype.chars() {
            if i.is_ascii_uppercase() { s.push(' '); }
            s.push(i);
        }
        format!("  {}, {}  ", self.name, s)
    }

    pub fn image(&self, index: usize) -> String {
        self.plugins[index].image.as_str()
    }

    pub fn install_plugins(&self, index: Vec<usize>) -> io::Result<()> {
        for i in index.iter() {
            self.plugins[i.clone()].install_files()?;
        }
        Ok(())
    }
}

impl Pattern {
    fn new() -> Pattern {
        Pattern {
            deps: Vec::new(),
            oper: String::new(),
            files: Vec::new(),
        }
    }

    pub fn install(cflags: Vec<ConditionFlag>) -> io::Result<()> {
        for i in cflags.iter() {

        }
        Ok(())
    }
}

pub fn read_install_instructions(src: &Path, dest: &Path) -> Vec<FomodGroup> {
    let file = dir::find_installfile(src);
    let raw = xml::read_xml_file(&file.as_str()).unwrap();
    read_groups(raw, src, dest)
}

fn read_groups(raw: xmltree::Element, src: &Path, dest: &Path) -> Vec<FomodGroup> {
    let groups = xml::get_children_r(raw, "group");
    let mut groups_v: Vec<FomodGroup> = Vec::new();

    for i in groups.iter() {

        groups_v.push(
            FomodGroup {
                name: i.attributes["name"].clone(),
                gtype: i.attributes["type"].clone(),
                plugins: read_plugins(i.clone(), src, dest),
            } );
    }
    groups_v
}

fn read_plugins(group: xmltree::Element, src: &Path, dest: &Path) -> Vec<FomodPlugin> {
    let plugins = xml::get_children_r(group, "plugin");
    let mut plugins_v: Vec<FomodPlugin> = Vec::new();

    for i in plugins.iter() {
        let mut plugin = FomodPlugin::new();
        plugin.name = i.attributes["name"].clone();
        
        match i.get_child("description") {
            Some(x) => match x.get_text() {
                Some(z) => { plugin.description = z.to_string(); }
                None => {}
            }
            None => {}
        }

        match i.get_child("image") {
            Some(x) => { plugin.image = src.clone().push_p(Path::from(&dir::fix_case(&x.attributes["path"]))); }
            None => {}
        }

        match i.get_child("typeDescriptor") {
            Some(x) => match x.get_child("type") {
                Some(z) => { plugin.type_desc = z.attributes["name"].clone(); }
                None => {}
            }
            None => {}
        }

        match i.get_child("files") {
            Some(x) => { plugin.files = read_files(x.clone(), src, dest); }
            None => {}
        }

        match i.get_child("conditionFlags") {
            Some(x) => { plugin.c_flags = Some(read_cflags(x.clone())); },
            None => {}
        }

        plugins_v.push(plugin);
    }

    plugins_v
}

fn read_files(plugin: xmltree::Element, src: &Path, dest: &Path) -> Vec<FomodFile> {
    let files = xml::get_children_all(plugin);
    let mut files_v: Vec<FomodFile> = Vec::new();

    for i in files.iter() {
        files_v.push( FomodFile {
            source: src.clone()
                .push_p(Path::from(&dir::fix_case(&i.attributes["source"]))),
            destination: dest.clone()
                .push_p(Path::from(&dir::fix_case(&i.attributes["destination"]))),
            ftype: i.name.clone(),
        } );
    }

    files_v
}

fn read_cflags(plugin: xmltree::Element) -> Vec<ConditionFlag> {
    let flags = xml::get_children_all(plugin);
    let mut flags_v: Vec<ConditionFlag> = Vec::new();

    for i in flags.iter() {
        flags_v.push( ConditionFlag {
            name: i.attributes["name"].clone(),
            active: false
        } );
    }

    flags_v
}

fn read_patterns(raw: xmltree::Element, src: &Path, dest: &Path) -> Vec<Pattern> {
    let patterns = xml::get_children_r(raw, "pattern");
    let mut patterns_v: Vec<Pattern> = Vec::new();

    for i in patterns.iter() {
        let mut pt = Pattern::new();

        match i.get_child("dependencies") {
            Some(x) => { 
                pt.deps = read_deps(x.clone()); 
                pt.oper = x.attributes["operator"].clone();
            },
            None => {}
        }

        match i.get_child("files") {
            Some(x) => { pt.files = read_files(x.clone(), src, dest); },
            None => {}
        }
    }
    patterns_v
}

fn read_deps(pattern: xmltree::Element) -> Vec<String> {
    let deps = xml::get_children_all(pattern);
    let mut deps_v: Vec<String> = Vec::new();

    for i in deps.iter() {
        deps_v.push(i.attributes["flag"].clone());
    }
    deps_v
}
