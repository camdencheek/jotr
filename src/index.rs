#![allow(dead_code)]

use anyhow;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
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
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e)  => {
                if e.kind() == io::ErrorKind::NotFound {
                    return Ok(Index::default())
                }
                return Err(anyhow::Error::new(e))
            },
        };
        let i: Index = serde_json::from_reader(file)?;
        Ok(i)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)?;
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

    pub fn delete(&mut self, entry: &IndexEntry) {
        if let Some(elem) = self.entries.get_mut(entry.id) {
            *elem = None
        }
    }


    pub fn get(&self, query: &str) -> Option<IndexEntry> {
        match usize::from_str_radix(&query, 10) {
            Ok(id) => self.get_by_id(id),
            Err(_) => self.get_by_name(&query)
        }
    }

    pub fn get_by_id(&self, id: usize) -> Option<IndexEntry> {
        self.entries.get(id).cloned().flatten()
    }

    pub fn get_by_name(&self, name: &str) -> Option<IndexEntry> {
        for entry in self.entries.iter().filter_map(|a| a.as_ref()) {
            if let Some(file_name) = entry.path.file_name() {
                if file_name == name {
                    return Some(entry.clone());
                }
            }

            if let Some(stem) = entry.path.file_stem() {
                if stem == name {
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

impl IndexEntry {
    pub fn summarize(&self) -> String {
        let f = match File::open(&self.path) {
            Ok(f) => f,
            Err(_) => return self.stem().to_string(),
        };

        let mut first_line  = String::new();
        if let Err(_) =  io::BufReader::new(f).read_line(&mut first_line) {
            return self.stem().to_string()
        }

        first_line.trim_start_matches('#').trim().to_string()
    }

    fn stem(&self) -> &str {
        return self
            .path
            .file_stem()
            .map(|x| x.to_str())
            .flatten()
            .unwrap_or("invalid_stem");
    }
}
