use clap::{ArgMatches};
use std::collections::{VecDeque};
use std::path::{PathBuf};

const DEFAULT_MAX_DEPTH: u8 = 255;
const DEFAULT_MAX_ITEMS: u32 = 0;

pub struct Config {
    relative: bool,
    max_items: u32,
    max_depth: u8,
    match_expr: String,
}

impl Config {
    pub fn from_arg_matches(matches: &ArgMatches) -> Config {
        let max_depth: u8 = match matches.value_of("max_depth") {
            Some(i) => String::from(i).parse().unwrap(),
            None => DEFAULT_MAX_DEPTH
        };
        let match_expr: String = matches.value_of("filename").unwrap_or_default().to_owned();
        let max_items: u32 = if matches.is_present("single") { 1 } else {
            match matches.value_of("max_items") {
                Some(i) => String::from(i).parse().unwrap(),
                None => DEFAULT_MAX_ITEMS
            }
        };
        let relative = matches.is_present("relative");
        Config { max_depth, match_expr, max_items, relative }
    }
}

struct DirNode {
    path: PathBuf,
    depth: u8
}

struct FileSearch<'a> {
    match_expr: &'a str,
    max_depth: u8,

    queue: VecDeque<DirNode>
}

impl<'a> FileSearch<'a> {
    fn new(match_expr: &'a str, root_dir: PathBuf, max_depth: u8) -> std::io::Result<FileSearch<'a>> {
        let mut queue = VecDeque::new();
        queue.push_front(DirNode {
            path: root_dir,
            depth: 0
        });
        Ok(FileSearch { match_expr, max_depth, queue })
    }
}

impl Iterator for FileSearch<'_> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current: DirNode;
        while !self.queue.is_empty() {
            current = self.queue.pop_front().unwrap();
            if current.depth > self.max_depth { break; }
            let dir_contents = std::fs::read_dir(&current.path);
            match dir_contents {
                Ok(read_dir) => {
                    for entry in read_dir {
                        let entry = entry.unwrap();
                        self.queue.push_back(DirNode {
                            path: entry.path(),
                            depth: current.depth + 1,
                        });
                        if entry.path().file_name().unwrap() == self.match_expr {
                            return Some(entry.path());
                        }
                    }
                },
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        eprintln!("Permission denied for: {:?}", current.path)
                    }
                }
            }
        };
        None
    }
}

pub fn run(cfg: Config) -> std::io::Result<()> {
    let root_dir = std::env::current_dir()?;
    let root_dir_string = root_dir.to_str().unwrap();
    let mut count: u32 = 0;
    for result in FileSearch::new(&cfg.match_expr, std::env::current_dir()?, cfg.max_depth)? {
        let path = result;
        let print_str = if cfg.relative { path.to_str().unwrap().replace(root_dir_string, "")}
            else { path.to_str().unwrap().to_owned() };
        println!("{}", print_str);
        count += 1;
        if count == cfg.max_items { break; }
    }
    Ok(())
}