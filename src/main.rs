mod folder;
mod result;

use crate::folder::Folder;
use crate::result::*;
use std::path::Path;

fn main() -> Result<()> {
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
        return Err(error!("could not find any directory to process"));
    }

    let mut folders: Vec<Folder> = vec![];
    for arg in args.into_iter() {
        let folder = Folder::new(&arg)?;
        if folder.is_git_repository() {
            println!("found git repository: {}", arg);
            folders.push(folder);
        } else {
            eprintln!("  not git repository: {}", arg);
            folders.push(folder);
        }
    }

    for folder in folders.into_iter() {
        println!("processing: {}", folder);
        folder.generate_summary_zip()?;
    }

    Ok(())
}
