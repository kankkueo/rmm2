
#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub active: bool,
}

#[derive(Clone)]
pub struct Fomod {
    pub plugin: Plugin,
    author: String,
    version: String,
    website: String,
    groups: String,
    description: String,
}



impl Fomod {
    fn to_str(&self) -> String {
        if self.plugin.active {
            format!("[Active]   {}", self.plugin.name)
        }
        else {
            format!("[Inactive] {}", self.plugin.name)
        }
    }

    pub fn activate(&mut self) {
        if self.plugin.active {
            self.plugin.active = false;
        }
        else {
            self.plugin.active = true;
        }
    }
}

pub fn to_strvec(src: &Vec<Fomod>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for i in src.iter() {
        vec.push(i.to_str());
    }
    vec
}

pub fn move_up(plugins: &mut Vec<Fomod>, idx: usize) {
    if idx > 0 {
        let temp = plugins[idx].clone();
        plugins[idx] = plugins[idx - 1].clone();
        plugins[idx - 1] = temp;
    }
}

pub fn move_down(plugins: &mut Vec<Fomod>, idx: usize) {
    if idx < plugins.len() - 1 {
        let temp = plugins[idx].clone();
        plugins[idx] = plugins[idx + 1].clone();
        plugins[idx + 1] = temp;
    }
}

