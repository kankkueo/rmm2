use std::fs;
use std::io;
use crate::paths::Path;
use crate::ui::utils::keyin;

pub mod xml;
pub mod dir;

/* These structs are trying to mimic the structure of a fomod
 * ModuleConfig file. The installation instructions are stored 
 * here. */
 
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
    c_flags: Vec<ConditionFlag>,
}

pub struct FomodGroup {
    name: String,
    gtype: String,
    plugins: Vec<FomodPlugin>,
}

pub struct FomodInstallStep {
    name: String,
    deps: Option<Vec<ConditionFlag>>,
    req_files: Vec<FomodFile>,
    pub groups: Vec<FomodGroup>,
}

pub struct ConditionFlag {
    name: String,
    state: bool,
}

pub struct Pattern {
    deps: Vec<ConditionFlag>,
    oper: String,
    files: Vec<FomodFile>,
}

impl FomodInstallStep {
    fn new() -> Self {
        Self {
            name: String::new(),
            deps: None,
            req_files: Vec::new(),
            groups: Vec::new(),
        }
    }

    pub fn check(&self, flags: &Vec<ConditionFlag>) -> bool {
        match &self.deps {
            Some(x) => { return ConditionFlag::check_all(&x, flags); }
            None => { return true; }
        }
    }
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
    fn new() -> Self {
        Self {
            name: String::new(),
            image: Path::new(),
            description: String::new(),
            type_desc: String::new(),
            files: Vec::new(),
            c_flags: Vec::new(),
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

    pub fn install_plugins(&self, index: &Vec<usize>) -> io::Result<()> {
        for i in index.iter() {
            self.plugins[i.clone()].install_files()?;
        }
        Ok(())
    }

    pub fn get_flags(&self, index: &Vec<usize>, flags_v: &mut Vec<ConditionFlag>) {
        for i in index.iter() {
            for j in self.plugins[i.clone()].c_flags.iter() {
                flags_v.push( ConditionFlag {
                    name: j.name.clone(),
                    state: j.state,
                } );
            }
        }
    }
} 

impl Pattern {
    fn new() -> Self {
        Self {
            deps: Vec::new(),
            oper: String::new(),
            files: Vec::new(),
        }
    }

    fn install_files(&self) -> io::Result<()> {
        for i in self.files.iter() {
            i.install()?;
        }

        Ok(())
    }

    fn check(&self, flags: &Vec<ConditionFlag>) -> bool {
        if self.oper == "And" || self.oper == "and" {
            ConditionFlag::check_all(&self.deps, flags)
        }
        else {
            ConditionFlag::check_one(&self.deps, flags)
        }
    }

    pub fn install(pvec: Vec<Self>, flags: Vec<ConditionFlag>) -> io::Result<()> {
        for i in pvec.iter() {
            if i.check(&flags) {
                i.install_files()?;
            }
        }

        Ok(())
    }
}

impl ConditionFlag {
    fn new() -> Self {
        Self {
            name: String::new(),
            state: true,
        }
    }

    fn check_fvec(vec: &Vec<Self>, flag: &Self) -> bool {
        for i in vec.iter() {
            if i.name == flag.name && i.state == flag.state { return true; }
        }
        false
    }

    fn check_all(deps: &Vec<Self>, flags: &Vec<Self>) -> bool {
        for i in deps.iter() {
            if !Self::check_fvec(flags, i) { return false; }
        }
        true
    }

    fn check_one(deps: &Vec<Self>, flags: &Vec<Self>) -> bool {
        for i in deps.iter() {
            if Self::check_fvec(flags, i) { return true; }
        }
        false
    }
}

/* All the stuff from here on has to do with reading the
 * installation steps from the ModuleConfig file */

pub fn read_install_instructions(src: &Path, dest: &Path) -> Vec<FomodInstallStep> {
    let file = dir::find_installfile(src);
    let raw = xml::read_xml_file(&file.as_str()).unwrap();
    read_install_steps(raw, src, dest)
}

pub fn read_conditional_patterns(src: &Path, dest: &Path) -> Option<Vec<Pattern>> {
    let file = dir::find_installfile(src);
    let raw = xml::read_xml_file(&file.as_str()).unwrap();
    let ps = read_patterns(raw, src, dest);

    if ps.len() > 0 { return Some(ps); }
    else { return None; }
}

pub fn read_install_steps(raw: xmltree::Element, src: &Path, dest: &Path) -> Vec<FomodInstallStep> {
    let steps = xml::get_children_r(raw, "installStep");
    let mut steps_v: Vec<FomodInstallStep> = Vec::new();

    for i in steps.iter() {
        let mut step = FomodInstallStep::new();
        step.name = i.attributes["name"].clone();
        step.groups = read_groups(i.clone(), src, dest);
        
        match i.get_child("requiredFiles") { //placehoder name
            Some(x) => { step.req_files = read_files(x.clone(), src, dest); },
            None => {}
        }

        match i.get_child("visible") {
            Some(x) => { step.deps = Some(read_deps(x.clone())); },
            None => {}
        }

        steps_v.push(step);
    }

    steps_v
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
            Some(x) => { plugin.c_flags = read_cflags(x.clone()); },
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

    let negv = vec![
        "off","Off", "OFF",
        "no","No", "NO",
        "n", "N",
    ];

    let posv = vec![
        "on", "On", "ON",
        "yes", "Yes", "YES",
        "y", "Y",
    ];

    for i in flags.iter() {
        let mut flag = ConditionFlag::new();
        flag.name = i.attributes["name"].clone();

        match i.get_text() {
            Some(x) => {
                for j in negv.iter() {
                    if x == j.clone() { flag.state = false; }
                }
                for j in posv.iter() {
                    if x == j.clone() { flag.state = true; }
                }
            },
            None => {}
        }

        flags_v.push(flag);
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

        patterns_v.push(pt);
    }
    patterns_v
}

fn read_deps(pattern: xmltree::Element) -> Vec<ConditionFlag> {
    let deps = xml::get_children_all(pattern);
    let mut deps_v: Vec<ConditionFlag> = Vec::new();

    let negv = vec![
        "off","Off", "OFF",
        "no","No", "NO",
        "n", "N",
    ];

    let posv = vec![
        "on", "On", "ON",
        "yes", "Yes", "YES",
        "y", "Y",
    ];

    for i in deps.iter() {
        let mut flag = ConditionFlag::new();
        flag.name = i.attributes["flag"].clone();
        let val = i.attributes["value"].clone();

        for j in negv.iter() {
            if val == j.clone() { flag.state = false; }
        }
        for j in posv.iter() {
            if val == j.clone() { flag.state = true; }
        }
 
        deps_v.push(flag);
    }
    deps_v
}
