#![feature(fs_walk)]
#![feature(path_ext)]
#![feature(metadata_ext)]
#![feature(dir_entry_ext)]
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
use std::io;

fn walk(directory: &Path, years: &mut BTreeMap<i32, u64>) -> io::Result<()> {
    for entry in try!(walk_dir(&directory)) {
        let dir = try!(entry);
        let meta = try!(dir.metadata());

        if !meta.is_file() {
            continue;
        }
        let mtime = meta.as_raw().mtime();
        let mtime_nsec = meta.as_raw().mtime_nsec();
        let utc = at_utc(Timespec::new(mtime, mtime_nsec as i32));
        let year = 1900 + utc.tm_year;

        *years.entry(year).or_insert(0) += 1;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut years: BTreeMap<i32, u64> = BTreeMap::new();

    let directory = match &args[..] {
        [_, ref path] => {
            Path::new(path)
        },
        _ => {
            panic!("usage: agealyzer </path/to/directory>")
        }
    };

    if let Err(_) = walk(&directory, &mut years) {
      panic!("failure walking");
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
