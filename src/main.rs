extern crate quicli;
extern crate walkdir;

mod defaults;

use quicli::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use walkdir::{DirEntry, WalkDir};

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Options {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let options = Options::from_args();
    let path = options.path;
    if path.is_file() {
        println!(
            "{} is not a directory.\n\nFor more information try --help",
            path.to_str().unwrap()
        );
        exit(1)
    }

    for entry in WalkDir::new(path).into_iter() {
        let entry = entry.unwrap();
        match prune(&entry) {
            false => continue,
            true => {
                println!("Removing {:?}", entry.path());
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(path).expect("failed to remove dir");
                } else {
                    std::fs::remove_file(path).expect("failed to remove file")
                }
            }
        }
    }

    exit(0);
}

fn prune(entry: &DirEntry) -> bool {
    if entry.path().is_dir() {
        let is_dir = defaults::dirs()
            .iter()
            .any(|s| *s == entry.path().file_stem().unwrap());

        return is_dir;
    }

    if is_hidden_file(entry) {
        return true;
    }

    if defaults::files()
        .iter()
        .any(|s| *s == entry.path().file_name().unwrap())
    {
        return true;
    }

    defaults::extensions()
        .iter()
        .any(|s| *s == entry.path().extension().unwrap())
}

fn is_hidden_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
