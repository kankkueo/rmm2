//use std::fs;
use std::fs::File;
use std::io::Read;
use encoding_rs_io::DecodeReaderBytes;
use xmltree::Element;


fn read_utf16(path: &str) -> String {
    let mut file = DecodeReaderBytes::new(File::open(path).expect("Could not open"));
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    buff
}

fn read_xml_tree(element: xmltree::Element, tree: &mut Vec<xmltree::Element>) {
    let children = get_children_all(element);
    for i in 0..children.len() {
        tree.push(children[i].clone());
        read_xml_tree(children[i].clone(), tree);
    }
}

pub fn get_children_all(element: xmltree::Element) -> Vec<xmltree::Element> {
    let mut child_v: Vec<xmltree::Element> = Vec::new();
    let children = &element.children;

    for i in 0..children.len() {
        match children[i].as_element() {
            Some(x) => child_v.push(x.clone()) ,
            None => continue,
        }
    }
    child_v
}

fn get_children(tree: Vec<xmltree::Element>, predicate: &str) -> Vec<xmltree::Element> {
    let mut children: Vec<xmltree::Element> = Vec::new();
    for i in 0..tree.len() {
        if tree[i].name == predicate {
            children.push(tree[i].clone());
        }
    }
    children
}

pub fn get_child_r(element: xmltree::Element, predicate: &str) -> Option<xmltree::Element> {
    let children = get_children_all(element);
    for i in 0..children.len() {
        if children[i].name == predicate {
            return Some(children[i].clone());
        }
        else {
            match get_child_r(children[i].clone(), predicate) {
                Some(x) => {return Some(x);},
                None => continue,
            }
        }
    }
    None
}

fn read_children_r(tree: &mut Vec<xmltree::Element>, element: xmltree::Element, predicate: &str) {
    let children = get_children_all(element);
    for i in 0..children.len() {
        if children[i].name == predicate {
            tree.push(children[i].clone());
        }
        read_children_r(tree, children[i].clone(), predicate);
    }
}

pub fn get_children_r(element: xmltree::Element, predicate: &str) -> Vec<xmltree::Element> {
    let mut children: Vec<xmltree::Element> = Vec::new();
    read_children_r(&mut children, element, predicate);
    children
}

pub fn turn_slashes(path: String) -> String {
    let mut string = String::new();
    for i in path.chars() {
        if i == '\\' {
            string.push('/');
        }
        else {
            string.push(i);
        }
    }
    string
}

pub fn read_xml_file(path: &str) -> xmltree::Element {
    Element::parse(read_utf16(path).as_bytes()).unwrap()
}



