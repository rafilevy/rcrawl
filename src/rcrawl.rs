use clap::{ArgMatches};
use std::collections::{VecDeque};
use std::path::{PathBuf};
use std::io::Result;

use std::thread;
use std::sync::{Arc, Mutex};

use crate::utils::{PathPrinter};

const DEFAULT_MAX_DEPTH: u8 = 255;
const DEFAULT_MAX_ITEMS: u32 = u32::MAX;
const DEFAULT_NUM_THREADS: u8 = 16;

pub struct Config {
    relative: bool,
    max_items: u32,
    max_depth: u8,
    match_expr: String,
    num_threads: u8
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
        let num_threads: u8 = match matches.value_of("threads") {
            Some(i) => String::from(i).parse().unwrap(),
            None => DEFAULT_NUM_THREADS
        };
        Config { max_depth, match_expr, max_items, relative, num_threads }
    }
}

struct DirNode {
    path: PathBuf,
    depth: u8
}

struct ConcurrentFileSearch<'a> {
    match_expr: &'a str,
    max_depth: u8,
    max_items: u32,
    relative: bool,
    num_threads: u8,

    search_queue: Arc<Mutex<VecDeque<DirNode>>>,
}

impl<'a> ConcurrentFileSearch<'a> {
    fn new(match_expr: &'a str, root_dir: PathBuf, max_depth: u8, max_items: u32, relative: bool, num_threads: u8) -> Result<ConcurrentFileSearch<'a>> {
        let search_queue = Arc::new(Mutex::new(VecDeque::new()));
        search_queue.lock().unwrap().push_front(DirNode {
            depth: 0,
            path: root_dir
        });
        Ok(ConcurrentFileSearch {match_expr, max_depth, max_items, relative, search_queue, num_threads})
    }
}

impl ConcurrentFileSearch<'_> {
    fn search(&mut self) {
        let path_printer = Arc::new(PathPrinter::new(self.relative).unwrap());
        let mut handles: Vec<std::thread::JoinHandle<()>> = vec!();
        let num_items : Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        for _ in 0..self.num_threads {
            let search_queue = Arc::clone(&self.search_queue);
            let path_printer = Arc::clone(&path_printer);
            let num_items = Arc::clone(&num_items);
            let match_expr = self.match_expr.to_owned();
            let max_depth = self.max_depth;
            let max_items = self.max_items;
            let handle = thread::spawn(move || {
                while (*num_items.lock().unwrap() < max_items) && !(*search_queue.lock().unwrap()).is_empty() {
                    let current = {
                        let mut search_queue = search_queue.lock().unwrap();
                        if (*search_queue).is_empty() {
                            break;
                        }
                        (*search_queue).pop_front().unwrap()
                    };
                    if current.depth > max_depth { break; }
                    let dir_contents = std::fs::read_dir(&current.path);
                    match dir_contents {
                        Ok(read_dir) => {
                            for entry in read_dir {
                                let entry = entry.unwrap();
                                if !entry.file_name().as_os_str().to_str().unwrap().starts_with(".") {
                                    (search_queue.lock().unwrap()).push_back(DirNode {
                                        path: entry.path(),
                                        depth: current.depth + 1,
                                    });
                                }
                                let mut num_items = num_items.lock().unwrap();
                                if entry.path().file_name().unwrap() == &match_expr[..] && *num_items < max_items {
                                    path_printer.print_path(entry.path());
                                    *num_items += 1;
                                }
                            }
                        },
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::PermissionDenied {
                                eprintln!("Permission denied for: {:?}", current.path)
                            }
                        }
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub fn run(cfg: Config) -> std::io::Result<()> {
    let root_dir = std::env::current_dir()?;
    ConcurrentFileSearch::new(&cfg.match_expr, root_dir, cfg.max_depth, cfg.max_items, cfg.relative, cfg.num_threads)?.search();
    Ok(())
}