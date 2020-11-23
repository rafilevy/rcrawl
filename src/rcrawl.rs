use clap::{ArgMatches};
use std::collections::{VecDeque};
use std::path::{PathBuf};
use std::io::Result;

use std::thread;
use std::sync::{Arc, Mutex};

use crate::utils::{PathPrinter};

const DEFAULT_MAX_DEPTH: u8 = 255;
const DEFAULT_MAX_ITEMS: u32 = 0;
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

struct FileSearch<'a> {
    match_expr: &'a str,
    max_depth: u8,
    relative: bool,

    queue: VecDeque<DirNode>
}

impl<'a> FileSearch<'a> {
    fn new(match_expr: &'a str, root_dir: PathBuf, max_depth: u8, relative: bool) -> Result<FileSearch<'a>> {
        let mut queue = VecDeque::new();
        queue.push_front(DirNode {
            path: root_dir,
            depth: 0
        });
        Ok(FileSearch { match_expr, max_depth, queue, relative })
    }
}

impl FileSearch<'_> {
    fn search(&mut self) {
        let path_printer = PathPrinter::new(self.relative).unwrap();

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
                            path_printer.print_path(entry.path());
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
    }
}

struct ConcurrentFileSearch<'a> {
    match_expr: &'a str,
    max_depth: u8,
    relative: bool,
    num_threads: u8,

    search_queue: Arc<Mutex<VecDeque<DirNode>>>,
}

impl<'a> ConcurrentFileSearch<'a> {
    fn new(match_expr: &'a str, root_dir: PathBuf, max_depth: u8, relative: bool, num_threads: u8) -> Result<ConcurrentFileSearch<'a>> {
        let search_queue = Arc::new(Mutex::new(VecDeque::new()));
        search_queue.lock().unwrap().push_front(DirNode {
            depth: 0,
            path: root_dir
        });
        Ok(ConcurrentFileSearch {match_expr, max_depth, relative, search_queue, num_threads})
    }
}

impl ConcurrentFileSearch<'_> {
    fn search(&mut self) {
        let path_printer = Arc::new(PathPrinter::new(self.relative).unwrap());

        let mut handles: Vec<std::thread::JoinHandle<()>> = vec!();
        for _ in 0..self.num_threads {
            let search_queue = Arc::clone(&self.search_queue);
            let path_printer = Arc::clone(&path_printer);
            let match_expr = self.match_expr.to_owned();
            let max_depth = self.max_depth;
            let handle = thread::spawn(move || {
                while !(*search_queue.lock().unwrap()).is_empty() {
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
                                    (*search_queue.lock().unwrap()).push_back(DirNode {
                                        path: entry.path(),
                                        depth: current.depth + 1,
                                    });
                                }
                                if entry.path().file_name().unwrap() == &match_expr[..] {
                                    path_printer.print_path(entry.path());
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
    if cfg.num_threads == 1 { FileSearch::new(&cfg.match_expr, root_dir, cfg.max_depth, cfg.relative)?.search(); } 
    else { ConcurrentFileSearch::new(&cfg.match_expr, root_dir, cfg.max_depth, cfg.relative, cfg.num_threads)?.search(); }
    Ok(())
}