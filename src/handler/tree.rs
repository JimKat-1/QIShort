use std::collections::HashMap;
use crate::key::Key;
// pub mod dir;

pub struct Func {
    arg: String,
    func: fn(&str)
}

pub enum TreeEntry {
    Tree(Tree),
    Func(Func),
    None
}

pub type Tree = HashMap<String, TreeEntry>;
