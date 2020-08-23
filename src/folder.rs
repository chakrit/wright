use crate::result::*;
use std::fmt::Display;
use std::fs::{canonicalize, metadata, File};
use std::iter::IntoIterator;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Folder {
    path: PathBuf,
}

impl Folder {
    pub fn new(base_path: &str) -> Result<Folder> {
        let canon_path = canonicalize(&base_path)?;
        let canon_path = canon_path
            .to_str()
            .ok_or_else(|| error!("failed to resolve path: {}", base_path))?;
        Ok(Folder {
            path: PathBuf::from(canon_path),
        })
    }

    pub fn is_git_repository(&self) -> bool {
        let git_path = self.path.join(".git");
        match metadata(&git_path) {
            Ok(attr) => attr.is_dir(),
            Err(_) => false,
        }
    }

    pub fn file_stats(&self) -> Result<Vec<(String, u32)>> {
        println!("processing: {:?}", self.path.to_str());

        let cfg = tokei::Config::default();
        let ignores = [".git", ".yarn", "target"];
        let mut langs = tokei::Languages::new();
        langs.get_statistics(&[&self.path], &ignores, &cfg);

        let mut results = Vec::<(String, u32)>::new();
        for (_, lang) in langs.into_iter() {
            let mut lang: tokei::Language = lang.summarise();
            lang.sort_by(tokei::Sort::Files);

            for r in lang.reports.into_iter() {
                let filename = r.name.to_string_lossy().to_string();
                results.push((filename, r.stats.code as u32));
            }
        }

        Ok(results)
    }

    pub fn generate_summary_zip(&self) -> Result<File> {
        let mut stats = self.file_stats()?;
        stats.sort_by(|p1, p2| p2.1.cmp(&p1.1));
        stats.truncate(20);
        for (lang, lines) in stats.iter() {
            println!("{} = {}", lang, lines);
        }

        let mut zip_name = self.path.to_string_lossy().to_string();
        zip_name.push_str(".zip");

        let file = File::create(&zip_name)
            .with_context(|| format!("failed to create zip archive: {}", &zip_name))?;
        {
            let mut zip = zip::ZipWriter::new(&file);

            for (num, (filename, _)) in stats.into_iter().enumerate() {
                let mut source_file = File::open(&filename)
                    .with_context(|| format!("failed to open: {}", &filename))?;

                let entry_name: PathBuf = filename.clone().into();
                let entry_name = format!(
                    "file {} - {}",
                    num + 1,
                    entry_name
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .ok_or_else(|| error!("cannot zip path: {}", &filename))?
                );

                zip.start_file(entry_name, zip::write::FileOptions::default())
                    .with_context(|| format!("problem writing zip entry for: {}", &filename))?;
                std::io::copy(&mut source_file, &mut zip)
                    .with_context(|| format!("problem writing zip entry for: {}", &filename))?;
            }

            zip.finish()
                .with_context(|| format!("problem finishing zip archive: {}", &zip_name))?;
        }
        Ok(file)
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}
