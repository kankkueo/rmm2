
#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pub active: bool,
}


/*
#[derive(Clone)]
pub struct Fomod {
    pub plugin: Plugin,
    pub author: String,
    pub version: String,
    pub website: String,
    pub description: String,
}
*/

impl Plugin {
    fn to_str(&self) -> String {
        if self.active {
            format!("[Active]   {}", self.name)
        }
        else {
            format!("[Inactive] {}", self.name)
        }
    }

    pub fn activate(&mut self) {
        if self.active {
            self.active = false;
        }
        else {
            self.active = true;
        }
    }

}

pub fn to_strvec(src: &Vec<Plugin>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for i in src.iter() {
        vec.push(i.to_str());
    }
    vec
}

pub fn move_up(plugins: &mut Vec<Plugin>, idx: usize) {
    if idx > 0 {
        let temp = plugins[idx].clone();
        plugins[idx] = plugins[idx - 1].clone();
        plugins[idx - 1] = temp;
    }
}

pub fn move_down(plugins: &mut Vec<Plugin>, idx: usize) {
    if idx < plugins.len() - 1 {
        let temp = plugins[idx].clone();
        plugins[idx] = plugins[idx + 1].clone();
        plugins[idx + 1] = temp;
    }
}

