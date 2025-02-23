use std::path;

pub fn get_root_file_path(file: &str) -> Result<path::PathBuf, String>
{
    let mut root_path = std::env::current_dir()
            .expect("Unable to read current working directory");

    root_path.push(file);
    return Ok(root_path);
}
