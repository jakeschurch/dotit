#[macro_use]
extern crate clap;

use std::env;
use std::fs::{self, DirBuilder};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Dotit Dotfiles Manager")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("inputs").multiple(true))
        .get_matches();

    let files: Vec<&str> = matches.values_of("inputs").unwrap().collect();

    let canonicalize_buffer = |filename: &str| -> PathBuf {
        let buf = PathBuf::from(filename);
        fs::canonicalize(buf).unwrap()
    };

    let dm = DotsManager::new();

    files
        .into_iter()
        .map(|filename| dm.store_file(canonicalize_buffer(filename)))
        .collect::<Vec<_>>();
    // dm.store_file(fs::canonicalize(buf).unwrap());
}

struct DotsManager {
    user_home: String,
    dots_home: String,
}

impl DotsManager {
    pub fn new() -> Self {
        let dots_home = match env::var("DOTFILES_HOME") {
            Ok(val) => val,
            Err(_) => panic!("Set `DOTFILES_HOME` before using"),
        };

        let user_home = match env::var("HOME") {
            Ok(val) => val,
            Err(e) => panic!("Set `HOME` before using"),
        };

        Self {
            user_home,
            dots_home,
        }
    }

    fn store_file<P: AsRef<Path>>(&self, path: P) {
        let src = path.as_ref();

        let dest = Path::new(&self.dots_home).join(src.strip_prefix(&self.user_home).unwrap());

        // Store file contents
        let data = fs::read(src).unwrap();

        // Create dots dest
        DirBuilder::new()
            .recursive(true)
            .create(dest.parent().unwrap());

        // Write to new file
        fs::write(&dest, data).unwrap();

        // delete src file
        fs::remove_file(src).expect("Failed to remove file");

        // Create symlink
        symlink(dest, src).expect("Failed to create symlink");
    }
}
