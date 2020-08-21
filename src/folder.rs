use crate::error::Error;
use crate::result::Result;
use std::fmt::Display;
use std::fs::{canonicalize, metadata};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Folder {
    path: PathBuf,
}

impl Folder {
    pub fn new(base_path: &str) -> Result<Folder> {
        let canon_path = canonicalize(&base_path)?;
        let canon_path = canon_path.to_str().ok_or_else(|| {
            Error::PathError(format!("failed to resolve path: {}", base_path).to_string())
        })?;
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

    pub fn lines_by_files(&self) -> Result<Vec<(String, u32)>> {
        use tokei::{Config, Languages, Sort};

        let cfg = Config::default();
        let ignores = [".git"];
        let mut langs = Languages::new();
        langs.get_statistics(&[&self.path], &ignores, &cfg);

        Ok(langs
            .into_iter()
            .flat_map(|(lang_type, lang)| {
                let mut lang = lang.summarise();
                lang.sort_by(Sort::Files);
                Some((lang.name, lang.reports[0].stats.code as u32))
            })
            .collect())
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)
    }
}
