#![feature(fs_walk)]
#![feature(path_ext)]
#![feature(metadata_ext)]
#![feature(slice_patterns)]
#![feature(libc)]

extern crate libc;
extern crate time;

use std::fs::walk_dir;
use std::fs::PathExt;
use std::path::Path;
use std::env;
use std::collections::BTreeMap;
use std::os::unix::fs::MetadataExt;
use std::iter::repeat;
use time::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut years: BTreeMap<i32, u64> = BTreeMap::new();
    let mut directory;

    match &args[..] {
        [ref name] => {
            panic!("usage: {} </path/to/directory>", name);
        },
        [_, ref path] => {
            directory = Path::new(path);
        },
        _ => {
            panic!("usage: agealyzer </path/to/directory>");
        }
    }

    for entry in walk_dir(&directory).unwrap() {
        match entry {
            Ok(dir) => {
                if !dir.path().as_path().is_file() {
                    continue;
                }
                // let stat = dir.path().as_path().metadata().unwrap().as_raw();
                // let mtime = stat.mtime();
                // let mtime_nsec = stat.mtime_nsec();
                let mtime = dir.path().as_path().metadata().unwrap().as_raw().mtime();
                let mtime_nsec = dir.path().as_path().metadata().unwrap().as_raw().mtime_nsec();
                let utc = at_utc(Timespec::new(mtime, mtime_nsec as i32));

                /* WTF, RUST? Why are your years off by 1900? */
                let year = 1900 + utc.tm_year;

                if !years.contains_key(&year) {
                    years.insert(year, 1);
                } else {
                    if let Some(x) = years.get_mut(&year) {
                        *x = *x + 1;
                    }
                }
            },
            Err(e) => println!("Shit! {:?}", e),
        }
    }

    let zero = 0u64;
    let max_hits = match years.values().max() {
        Some(m) => m,
        None => &zero, 
    };
    for y in years.keys() {
        let hits = match years.get(&y) {
            Some(h) => h,
            None => &zero,
        };
        let p = (40.0 * *hits as f64) / *max_hits as f64;
        let pf = repeat("=").take(p as usize).collect::<String>();
        println!("{0}: |{1: <40}| ({2})", y, pf, hits);
    }
}
