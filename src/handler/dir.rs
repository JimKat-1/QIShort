use std::fs;
use std::path::Path;
use std::error::Error;
use crate::lazy_static::lazy_static;
use crate::key::Key;

// TODO: This only works for systems complying to the XDG standard.
// static mut ROOT: PathBuf = std::env::var("XDG_CONFIG_DIR").into();

// Not necessarily the actual directory of the process.
pub fn cwd(cd: Option<&Path>) -> &'static Path {
    lazy_static! {
        static ref cwd: Box<Path> = Box::new(Path::new("/home/jimkat/.config/qishort/root/"));
    }

    &cwd
}

pub fn get_dir_entries(dir: &Path) -> Result<Vec<String>, Box<dyn Error>> {
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

pub fn handle(key: Key) {
    
}
