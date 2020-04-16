//! Defines cache file format

use crate::repository::Repository;
use anyhow::Result;
use chrono::{DateTime, Local};
use file_lock::FileLock;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::Path};

// inner representation of cache format.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CacheData {
    pub repositories: BTreeSet<Repository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    timestamp: DateTime<Local>,
    inner: Option<CacheData>,
}

#[derive(Debug)]
pub struct SharedCache {
    inner: Cache,
    lock: FileLock,
}

impl SharedCache {
    pub fn new(cache_path: &Path) -> Result<Self> {
        if cache_path.exists() {
            // force the lock to drop
            let cache: Cache = {
                let mut lock = FileLock::lock(cache_path.to_string_lossy().as_ref(), true, false)?;
                serde_json::from_reader(&mut lock.file)?
            };

            let lock = FileLock::lock(cache_path.to_string_lossy().as_ref(), true, true)?;

            Ok(SharedCache {
                inner: cache,
                lock: lock,
            })
        } else {
            let lock = FileLock::lock(cache_path.to_string_lossy().as_ref(), true, true)?;

            Ok(SharedCache {
                inner: Cache {
                    timestamp: Local::now(),
                    inner: None,
                },
                lock: lock,
            })
        }
    }

    pub fn get_opt(&self) -> Option<&CacheData> {
        self.inner.inner.as_ref()
    }

    pub fn get_mut(&mut self) -> &mut CacheData {
        if self.inner.inner.is_none() {
            self.inner.inner = Some(Default::default());
        }
        self.inner.inner.as_mut().unwrap()
    }

    pub fn dump(&mut self) -> Result<()> {
        self.inner.timestamp = Local::now();
        serde_json::to_writer_pretty(&self.lock.file, &self.inner).map_err(Into::into)
    }
}
