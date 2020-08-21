mod error;
mod folder;
mod result;

use crate::error::Error;
use crate::folder::Folder;
use crate::result::Result;
use std::path::Path;

fn main() {
    match really_main() {
        Err(err) => panic!("error: {}", err),
        Ok(_) => println!("finished."),
    }
}

fn really_main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        let cwd = Path::new(".").canonicalize()?;
        let dir = cwd.read_dir()?;
        args = dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect();
    } else {
        args = args
            .iter()
            .filter_map(|arg| Path::new(&arg).canonicalize().ok())
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect();
    }

    if args.len() == 0 {
        return Err(Error::UsageError("need at least 1 module to process."));
    }

    let mut folders: Vec<Folder> = vec![];
    for arg in args.into_iter() {
        let folder = Folder::new(&arg);
        match folder {
            Err(err) => {
                eprintln!("cannot access {}: {}", arg, err);
                continue;
            }
            Ok(f) => {
                if f.is_git_repository() {
                    println!("found git repository: {}", arg);
                    folders.push(f);
                }
            }
        }
    }

    for folder in folders.into_iter() {
        let stats = folder.lines_by_files().unwrap();
        for (lang, lines) in stats {
            println!("{} = {}", lang, lines);
        }
    }

    Ok(())
}
