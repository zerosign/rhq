use anyhow::Result;
use shellexpand;
use std::borrow::Borrow;
use std::path::{Path, PathBuf};

pub fn make_path_buf<S: AsRef<str>>(s: S) -> Result<PathBuf> {
    shellexpand::full(s.as_ref())
        .map(|s| PathBuf::from(s.borrow() as &str))
        .map_err(Into::into)
}

#[cfg(windows)]
pub fn canonicalize_pretty<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref()
        .canonicalize()
        .map_err(Into::into)
        .map(|path| {
            path.to_string_lossy()
                .trim_start_matches(r"\\?\")
                .replace(r"\", "/")
        })
        .map(|s| PathBuf::from(s))
}

#[cfg(not(windows))]
pub fn canonicalize_pretty<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref().canonicalize().map_err(Into::into)
}

pub trait StrSkip {
    fn skip(&self, n: usize) -> &str;
}

impl StrSkip for str {
    fn skip(&self, n: usize) -> &str {
        let mut s = self.chars();
        for _ in 0..n {
            s.next();
        }
        s.as_str()
    }
}

#[test]
fn test_skipped_1() {
    assert_eq!("hoge".skip(1), "oge");
    assert_eq!("あいueo".skip(1), "いueo");
}

pub mod process {
    use std::process::{Command, Stdio};

    pub fn inherit(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::inherit());
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command
    }

    pub fn piped(name: &str) -> Command {
        let mut command = Command::new(name);
        command.stdin(Stdio::null());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command
    }
}
