extern crate chrono;
extern crate walkdir;

use std::io;
use std::env;
use std::path::Path;
use std::collections::BTreeMap;
use std::time::UNIX_EPOCH;
use std::iter::repeat;

use chrono::{NaiveDateTime::from_timestamp, Datelike};
use walkdir::WalkDir;

fn walk(directory: &Path) -> io::Result<BTreeMap<i32, u64>> {
    let mut years: BTreeMap<i32, u64> = BTreeMap::new();
    for entry in WalkDir::new(&directory) {
        let meta = entry?
            .path()
            .symlink_metadata()?;

        if !meta.is_file() {
            continue;
        }

        if let Ok(seconds) = meta.modified()?.duration_since(UNIX_EPOCH) {
            let year = from_timestamp(seconds.as_secs() as i64, 0).date().year();
            *years.entry(year).or_insert(0) += 1;
        }
    }
    Ok(years);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let directory = match args.len() {
        2 => Path::new(&args[1]),
        _ => panic!("usage: agealyzer </path/to/directory>"),
    };

    let years = walk(&directory).expect("at least an empty collection");

    let max_hits = years.values().max().expect("should be at least one int");
    for y in years.keys() {
        let hits = years.get(&y).expect("should be a value here");
        let p = (60.0 * *hits as f64) / *max_hits as f64;
        let pf = repeat("=").take(p as usize).collect::<String>();
        println!("{0}: |{1: <60}| ({2})", y, pf, hits);
    }
}
