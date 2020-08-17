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
        Ok(_) => println!("done."),
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

    let (ok_folders, err_folders): (Vec<Result<Folder>>, Vec<Result<Folder>>) = args
        .into_iter()
        .map(|a| Folder::new(&a))
        .partition(|f| f.is_ok());
    let ok_folders: Vec<_> = ok_folders.into_iter().flat_map(|f| f.ok()).collect();
    let err_folders: Vec<_> = err_folders.into_iter().flat_map(|f| f.err()).collect();

    for err in err_folders.iter() {
        println!("could not access folder: {}", err);
    }
    for f in ok_folders.iter() {
        if f.is_git_repository() {
            println!("git repo detected: {}", &f);
        } else {
            println!("not a git repo: {}", &f);
        }
    }

    Ok(())
}
