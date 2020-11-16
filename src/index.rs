#![allow(dead_code)]

use anyhow;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    entries: Vec<Option<IndexEntry>>,
}

impl Default for Index {
    fn default() -> Self {
        Index {
            entries: Vec::new(),
        }
    }
}

impl Index {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let i: Index = serde_json::from_reader(file)?;
        Ok(i)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = OpenOptions::new().create(true).write(true).open(&path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn add(&mut self, path: &Path) {
        let empty_idx = self.entries.iter().enumerate().find(|a| a.1.is_none());

        if let Some((i, _)) = empty_idx {
            self.entries[i] = Some(IndexEntry {
                id: i,
                last_opened: Utc::now(),
                path: path.into(),
            });
        } else {
            self.entries.push(Some(IndexEntry {
                id: self.entries.len(),
                last_opened: Utc::now(),
                path: path.into(),
            }));
        }
    }

    pub fn sorted_most_recent(&self, limit: Option<usize>) -> Vec<IndexEntry> {
        let mut filtered: Vec<IndexEntry> = self.entries.iter().filter_map(|i| i.clone()).collect();
        filtered.sort_by(|a, b| a.last_opened.cmp(&b.last_opened));
        if let Some(count) = limit {
            filtered.truncate(count)
        }
        filtered
    }

    pub fn delete(&mut self, id: usize) {
        if let Some(elem) = self.entries.get_mut(id) {
            *elem = None
        }
    }

    pub fn get(&self, id: usize) -> Option<IndexEntry> {
        self.entries.get(id).cloned().flatten()
    }

    pub fn get_by_name(&self, name: &str) -> Option<IndexEntry> {
        for entry in self.entries.iter().filter_map(|a| a.as_ref()) {
            if let Some(file_name) = entry.path.file_name() {
                if file_name == OsString::from(&name) {
                    return Some(entry.clone());
                }
            }
        }
        return None;
    }

    pub fn touch(&mut self, id: usize) {
        if let Some(e) = self.entries.get_mut(id) {
            if let Some(e) = e {
                e.last_opened = Utc::now();
            }
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct IndexEntry {
    pub id: usize,
    pub last_opened: DateTime<Utc>,
    pub path: PathBuf,
}
