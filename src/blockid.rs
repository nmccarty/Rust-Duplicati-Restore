use base64::*;
use serde::Deserialize;
use serde_json::{Result, Value};

pub struct BlockToFile {
    pub(self) id: i32,
    pub(self) block_id: String,
    pub(self) file_num: i64,
}

impl BlockToFile {
    pub fn new(id: i32, block_id: String, file_num: i64) -> BlockToFile {
        BlockToFile {
            id,
            block_id,
            file_num,
        }
    }
}

pub fn base64_url_to_plain(url: &str) -> String {
    base64::encode(&base64::decode_config(url, base64::URL_SAFE).unwrap())
}

pub enum FileType {
    File {
        hash: String,
        size: i64,
        time: String,
    },
    Folder {
        metablockhash: String,
    },
    SymLink,
}

pub struct FileEntry {
    metahash: String,
    metasize: i64,
    file_type: FileType,
    block_lists: Vec<String>,
}

impl FileEntry {
    pub(self) fn from_ientry(ientry: &IEntry) -> FileEntry {
        let metahash = ientry.metahash.clone();
        let metasize = ientry.metasize;
        let block_lists = if let Some(blocks) = &ientry.blocklists {
            blocks.clone()
        } else {
            Vec::new()
        };
        let file_type = match ientry.filetype.as_ref() {
            "File" => FileType::File {
                hash: ientry.hash.clone().unwrap(),
                size: ientry.size.unwrap(),
                time: ientry.time.clone().unwrap(),
            },
            "Folder" => FileType::Folder {
                metablockhash: ientry.metablockhash.clone().unwrap(),
            },
            _ => FileType::SymLink,
        };

        FileEntry {
            metahash,
            metasize,
            file_type,
            block_lists,
        }
    }

    pub fn is_file(&self) -> bool {
        match self.file_type {
            FileType::File { .. } => true,
            _ => false,
        }
    }
}

#[derive(Deserialize)]
pub(self) struct IEntry {
    pub(self) hash: Option<String>,
    pub(self) metablockhash: Option<String>,
    pub(self) metahash: String,
    pub(self) metasize: i64,
    pub(self) path: String,
    #[serde(rename = "type")]
    pub(self) filetype: String,
    pub(self) size: Option<i64>,
    pub(self) time: Option<String>,
    pub(self) blocklists: Option<Vec<String>>,
}

/// Accepts the dlist as a string (must be read in first)
/// Returns a Vec of FileEntrys
pub fn parse_dlist(dlist: &str) -> Vec<FileEntry> {
    let mut file_entries = Vec::new();
    let entry_list: Vec<IEntry> = serde_json::from_str(dlist).unwrap();
    for entry in entry_list {
        file_entries.push(FileEntry::from_ientry(&entry));
    }

    file_entries
}
