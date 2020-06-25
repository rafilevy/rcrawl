use clap::{ArgMatches};
use std::collections::{VecDeque};
use std::path::{PathBuf};

const DEFAULT_MAX_DEPTH: u8 = 50;
const DEFAULT_MAX_ITEMS: u8 = 10;

pub struct Config {
    relative: bool,
    max_items: u8,
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
        let max_items: u8 = if matches.is_present("single") { 1 } else {
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
            if current.depth < self.max_depth && current.path.is_dir() {
                let dir_contents = std::fs::read_dir(&current.path);
                match dir_contents {
                    Ok(read_dir) => {
                        for entry in read_dir {
                            let entry = entry.unwrap();
                            if entry.path().file_name().unwrap() == self.match_expr {
                                return Some(entry.path());
                            }
                            self.queue.push_front(DirNode {
                                path: entry.path(),
                                depth: current.depth + 1,
                            });
                        }
                    },
                    Err(_) => {
                        eprintln!("rcrawl: Permission denied for {:?}", current.path)
                    }
                }
            }
        };
        None
    }
}

pub fn run(cfg: Config) -> std::io::Result<()> {
    let root_dir = std::env::current_dir()?;
    let mut count: u8 = 0;
    for result in FileSearch::new(&cfg.match_expr, root_dir, cfg.max_depth)? {
        if count == cfg.max_items { break; }
        let path = result;
        // let path = if !cfg.relative { result }
        // else {
        //     let mut cmp_path = PathBuf::new();
        //     let mut path = PathBuf::new();
        //     for part in result.iter() {
        //         cmp_path.push(part);
        //         if cmp_path.gt(&root_dir) {
        //             path.push(part);
        //         }
        //     }
        //     path
        // };
        println!("{}", path.to_str().unwrap().replace(" ", "\\ "));
        count += 1;
    }
    Ok(())
}