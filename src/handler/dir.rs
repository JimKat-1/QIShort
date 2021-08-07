use std::fs;
use std::path::Path;
use std::error::Error;

// TODO: This only works for systems complying to the XDG standard.
// static mut ROOT: PathBuf = std::env::var("XDG_CONFIG_DIR").into();

// Not necessarily the actual directory of the process.
fn cwd() {
    static mut cwd: &mut Path;
    //cwd = &mut Path::new("./").into<&mut Path;
}

pub fn get_entries() -> Result<Vec<String>, Box<dyn Error>> {
    let mut entries = Vec::<String>::new();

    let dir = Path::new("/");
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            entries.push(entry?.file_name()
                               .into_string()
                               .or_else(|f| Err(format!("Invalid directory entry: {:?}", f)))?);
        }
    }

    Ok(entries)
}

pub fn handle() {
    
}
