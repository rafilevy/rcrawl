use clap::{ArgMatches};
use std::collections::{VecDeque};
use std::path::{PathBuf};
use std::fs;
use std::env;

const DEFAULT_MAX_DEPTH: u8 = 50;
const DEFAULT_MAX_ITEMS: u8 = 10;

pub struct Config {
    full: bool,
    max_items: u8,
    max_depth: u8,
    query: String,
}

impl Config {
    pub fn from_arg_matches(matches: &ArgMatches) -> Config {
        let max_depth: u8 = match matches.value_of("max_depth") {
            Some(i) => String::from(i).parse().unwrap(),
            None => DEFAULT_MAX_DEPTH
        };
        let query: String = matches.value_of("dirname").unwrap_or_default().to_owned();
        let max_items: u8 = if matches.is_present("single") { 1 } else {
            match matches.value_of("max_items") {
                Some(i) => String::from(i).parse().unwrap(),
                None => DEFAULT_MAX_ITEMS
            }
        };
        let full = matches.is_present("full");
        Config { max_depth, query, max_items, full }
    }
}

struct DirNode {
    path: Box<PathBuf>,
    depth: u8
}

fn find_dir(name: &str, max_depth: u8, max_items: u8) -> std::io::Result<Vec<PathBuf>> {

    let mut results: Vec<PathBuf> = Vec::new();
    let mut found = 0;

    let mut queue: VecDeque<DirNode> = VecDeque::new();

    queue.push_front(DirNode {
        path: Box::new(env::current_dir()?),
        depth: 0
    });

    let mut current: DirNode;
    while !queue.is_empty() && found < max_items {
        current = queue.pop_front().unwrap();
        if current.depth < max_depth && current.path.is_dir() {
            for entry in std::fs::read_dir(current.path.to_path_buf())? {
                let entry : fs::DirEntry = entry?;
                if entry.path().file_name().unwrap().eq(name) {
                    results.push(entry.path().to_path_buf());
                    found += 1;
                    if found == max_items { break; }
                }
                queue.push_front(DirNode {
                    path: Box::new(entry.path()),
                    depth: current.depth + 1,
                });
            }
        }
    };

    Ok(results)
}

pub fn run(cfg: Config) -> std::io::Result<()> {
    let results = find_dir(&cfg.query, cfg.max_depth, cfg.max_items)?;
    if results.is_empty() {
        eprintln!("rcrawl: No files or directory {} was found", cfg.query)
    } else {
        let cwd = env::current_dir().unwrap();
        for result in results {
            let path = if cfg.full { result }
            else {
                let mut cmp_path = PathBuf::new();
                let mut path = PathBuf::new();
                for part in result.iter() {
                    cmp_path.push(part);
                    if cmp_path.gt(&cwd) {
                        path.push(part);
                    }
                }
                path
            };
            println!("{}", path.to_str().unwrap().replace(" ", "\\ "))
        }
    }
    Ok(())
}