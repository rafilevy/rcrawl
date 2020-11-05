use std::path::{PathBuf};

pub fn print_path(path: PathBuf) {
    let print_str = path.to_str().unwrap().to_owned();
    println!("{}", print_str);
}

pub fn print_path_relative(path: PathBuf) -> Result<(), std::io::Error> {
    let root_dir = std::env::current_dir()?;
    let root_dir_string = root_dir.to_str().unwrap();
    let print_str = path.to_str().unwrap().replace(root_dir_string, "");
    println!("{}", print_str);
    Ok(())
}