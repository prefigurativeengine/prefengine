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

use dirs_next;

pub fn get_global_data_path(reticulum_subdir: bool) -> Result<path::PathBuf, String>
{
    let path_r: Option<path::PathBuf> = dirs_next::home_dir();
    if let None = path_r {
        return Err("dirs_next returned no home directory".to_owned())
    } else {
        let mut path = path_r.unwrap();
        path.push(".prefengine");
        if reticulum_subdir { path.push("reticulum") }
        
        Ok(path)
    }
}