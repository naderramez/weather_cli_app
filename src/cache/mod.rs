use std::{fs, io::Result, path::PathBuf};

pub fn save_to_cache(data: &str) -> Result<()> {
    let path = PathBuf::from("./cache.json");
    fs::write(path, data)?;
    Ok(())
}

pub fn load_from_cache() -> Result<Option<String>> {
    let path = PathBuf::from("./cache.json");
    if path.exists() {
        let content = fs::read_to_string(path)?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}
