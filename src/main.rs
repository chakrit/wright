mod error;
mod result;

use crate::result::Result;
use std::path::Path;

fn main() {
    match really_main() {
        Err(err) => println!("error: {}", err),
        Ok(_) => println!("done."),
    }
}

fn really_main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let modules: Vec<String>;

    if args.len() == 0 {
        let cwd = Path::new(".").canonicalize()?;
        let dir = cwd.read_dir()?;
        modules = dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect();
    } else {
        modules = args
            .iter()
            .filter_map(|arg| Path::new(&arg).canonicalize().ok())
            .filter_map(|p| p.to_str().map(|s| s.to_string()))
            .collect();
    }

    for m in modules {
        println!("detected modules: {}", m);
    }

    Ok(())
}
