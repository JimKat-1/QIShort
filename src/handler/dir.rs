use std::fs;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::error::Error;
use crate::lazy_static::lazy_static;
use super::tree::*;

// Not necessarily the actual directory of the process.
// pub fn cwd(cd: Option<&Path>) -> Result<&'static PathBuf, Box<dyn Error>> {
//     lazy_static! {
//         // TODO: This only works for systems complying to the XDG standard.
//         static ref cwd: PathBuf = match std::env::var("XDG_CONFIG_DIR") {
//             Ok(dir) => dir.into().push("qishort"),
//             Err(_) => match std::env::var("HOME") {
//                 Ok(dir) => dir.push(".config/qishort"),
//                 Err(err) => return Err(err)
//             }
//         };
//     }

//     if let Some(dir) = cd {
//         cwd.push(dir);
//         if !cwd.is_dir() {
//             for i in dir.iter(){
//                 cwd.pop();
//             }
//         }
//     }

//     Ok(&cwd)
// }

pub fn get_dir_entries(dir: &Path) -> Result<Vec<OsString>, Box<dyn Error>> {
    let mut entries = Vec::<String>::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            entries.push(entry?.file_name()
                               .into_string()
                               .or_else(|f| Err(format!("Invalid directory entry: {:?}", f)))?);
        }
    }

    Ok(entries)
}

fn fill_tree_from_dir(dir: PathBuf, tree: &mut Tree) {
    let mut dirent = get_dir_entries(dir)?;

    for i in dirent {
        dir.push(i);

        if dir.is_dir() {
            let mut tree = Tree::new();
            fill_tree_from_dir(dir, tree);
            tree.insert(i, TreeEntry::Tree(tree));
        } else {
            // TODO
            tree.insert(i, TreeEntry::Func(Func {}));
        }

        dir.pop();
    }
}
