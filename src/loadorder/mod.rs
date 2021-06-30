
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
            format!("[ X ]  {}", self.name)
        }
        else {
            format!("[   ]  {}", self.name)
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
    
    fn from(src: &str) -> Plugin {
        Plugin {
            name: src.to_string(),
            active: false,
        }
    }
}

pub fn to_plgvec(src: Vec<String>) -> Vec<Plugin> {
    let mut pvec: Vec<Plugin> = Vec::new();
    for i in src.iter() {
        pvec.push(Plugin::from(i)) ;
    }
    pvec
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

pub fn any_active(plugins: &Vec<Plugin>) -> bool {
    for i in plugins.iter() {
        if i.active {return true;}
    }
    false
}

pub fn get_active(plugins: &Vec<Plugin>) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    for i in 0..plugins.len() {
        if plugins[i].active {v.push(i);}
    }
    v
}
