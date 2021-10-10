use std::path::{PathBuf, Path};

pub struct PathPrinter {
    relative_length: usize
}

impl PathPrinter {
    pub fn new(relative: bool, root_dir: &Path) -> Result<PathPrinter, std::io::Error> {
        if relative {
            let relative_length = root_dir.to_str().unwrap_or_default().len();
            return Ok( PathPrinter {relative_length} );
        }
        Ok(PathPrinter {relative_length:0})
    }

    pub fn print_path(&self, path: PathBuf) {
        let print_str = &path.to_str().unwrap()[self.relative_length..];
        println!("{}", print_str);
    }
}