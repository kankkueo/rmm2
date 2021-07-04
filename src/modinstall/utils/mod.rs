
pub mod xml;
pub mod dir;

//use crate::files;
use crate::paths::Path;

pub struct FomodFile {
    pub source: Path,
    pub destination: Path,
    pub ftype: String,
}

pub struct FomodPlugin {
    pub name: String,
    pub image: Path,
    pub description: String,
    pub type_desc: String,
    pub files: Vec<FomodFile>,
}

pub struct FomodGroup {
    pub name: String,
    pub gtype: String,
    pub plugins: Vec<FomodPlugin>,
}

impl FomodPlugin {
    fn new() -> FomodPlugin {
        FomodPlugin {
            name: String::new(),
            image: Path::new(),
            description: String::new(),
            type_desc: String::new(),
            files: Vec::new(),
        }
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
}

pub fn read_install_instructions(src: &Path) -> Vec<FomodGroup> {
    let file = dir::find_installfile(src);
    let raw = xml::read_xml_file(&file.as_str()).unwrap();
    read_groups(raw)
}

fn read_groups(raw: xmltree::Element) -> Vec<FomodGroup> {
    let groups = xml::get_children_r(raw, "group");
    let mut groups_v: Vec<FomodGroup> = Vec::new();

    for i in groups.iter() {

        groups_v.push(
            FomodGroup {
                name: i.attributes["name"].clone(),
                gtype: i.attributes["type"].clone(),
                plugins: read_plugins(i.clone()),
            } );
    }
    groups_v
}

fn read_plugins(group: xmltree::Element) -> Vec<FomodPlugin> {
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
            Some(x) => { plugin.image = Path::from(&x.attributes["path"]); }
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
            Some(x) => { plugin.files = read_files(x.clone()); }
            None => {}
        }

        plugins_v.push(plugin);
    }

    plugins_v
}

fn read_files(plugin: xmltree::Element) -> Vec<FomodFile> {
    let files = xml::get_children_all(plugin);
    let mut files_v: Vec<FomodFile> = Vec::new();

    for i in files.iter() {
        files_v.push( FomodFile {
            source: Path::from(&i.attributes["source"]),
            destination: Path::from(&dir::fix_case(&i.attributes["destination"])),
            ftype: i.name.clone(),
        } );
    }

    files_v
}

