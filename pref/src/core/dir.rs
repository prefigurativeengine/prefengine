use std::path;

pub fn get_root_file_path(file: &str) -> Result<path::PathBuf, String>
{
    let root_path_r = std::env::current_dir();
    if let Err(err) = root_path_r {
        return Err(err.to_string())
    } else {
        let mut root_path = root_path_r.unwrap();
        root_path.push(file);
        
        Ok(root_path)
    }
}
