mod folder;
mod result;

use crate::folder::Folder;
use crate::result::*;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let folder_paths: Vec<String>;

    if args.len() == 0 {
        let cwd = Path::new(".")
            .canonicalize()
            .with_context(|| "could not determine cwd")?
            .to_string_lossy()
            .to_string();
        folder_paths = vec![cwd];
    } else {
        folder_paths = args
            .iter()
            .filter_map(|arg| {
                Path::new(&arg)
                    .canonicalize()
                    .with_context(|| format!("failed to find canonical path of: {}", &arg))
                    .ok()
            })
            .map(|p| p.to_string_lossy().to_string())
            .collect();
    }

    if folder_paths.len() == 0 {
        return Err(error!("could not find any directory to process"));
    }

    let mut folders: Vec<Folder> = vec![];
    for p in folder_paths.into_iter() {
        let folder = Folder::new(&p)?;
        if folder.is_git_repository() {
            println!("found git repository: {}", &p);
            folders.push(folder);
        } else {
            eprintln!("  not git repository: {}", &p);
            folders.push(folder);
        }
    }

    for folder in folders.into_iter() {
        println!("processing: {}", folder);
        folder.generate_summary_zip()?;
    }

    Ok(())
}
