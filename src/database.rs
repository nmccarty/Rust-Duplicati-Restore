use crate::blockid::*;
use pbr::ProgressBar;
use rusqlite::types::FromSql;
use rusqlite::*;
use serde::Deserialize;
use serde_json::{Result, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use zip;

#[derive(Deserialize)]
struct Manifest {
    #[serde(rename = "Version")]
    pub(self) version: i64,
    #[serde(rename = "Created")]
    pub(self) created: String,
    #[serde(rename = "Encoding")]
    pub(self) encoding: String,
    #[serde(rename = "Blocksize")]
    pub(self) block_size: i64,
    #[serde(rename = "BlockHash")]
    pub(self) block_hash: String,
    #[serde(rename = "FileHash")]
    pub(self) file_hash: String,
    #[serde(rename = "AppVersion")]
    pub(self) app_version: String,
}

pub struct DB {
    conn: Connection,
    manifest: Manifest,
}

impl DB {
    pub fn new(file: &str, manifest: &str) -> DB {
        let conn = Connection::open(file).unwrap();
        conn.execute("PRAGMA temp_store = memory", NO_PARAMS)
            .unwrap();
        conn.execute("PRAGMA page_size = 16384", NO_PARAMS).unwrap();
        conn.execute("PRAGMA cache_size = 2048", NO_PARAMS).unwrap();
        let manifest: Manifest = serde_json::from_str(manifest).unwrap();
        DB { conn, manifest }
    }

    pub fn create_block_id_to_filenames(
        mut self,
        number_to_name: &BTreeMap<usize, String>,
    ) -> Self {
        let conn = &mut self.conn;

        // Create Block ID -> File Number table and empty it out if it exists
        conn.execute(
            "create table if not exists BlockIdToFile (
                     BlockId TEXT,
                     FileNum INTEGER)",
            NO_PARAMS,
        )
        .unwrap();
        conn.execute(
            "create index if not exists IxBlockId on BlockIdToFile(BlockId)",
            NO_PARAMS,
        )
        .unwrap();
        conn.execute("delete from BlockIdToFile where 1", NO_PARAMS)
            .unwrap();

        // Iterate through dblocks, adding them to the db
        let mut pb = ProgressBar::new(number_to_name.len() as u64);
        for (num, path) in number_to_name.iter() {
            let tx = conn.transaction().unwrap();
            // Open zip file
            let file = File::open(&Path::new(path)).unwrap();
            let mut zip = zip::ZipArchive::new(file).unwrap();
            // Iterate through contents and add names to database
            for i in 0..zip.len() {
                let inner_file = zip.by_index(i).unwrap();
                let name = base64_url_to_plain(inner_file.name());
                // Add name to database
                tx.execute(
                    "insert into BlockIdToFile (BlockId, FileNum) VALUES (?1, ?2)",
                    &[&name, &num.to_string()],
                )
                .unwrap();
            }
            pb.inc();
            tx.commit().unwrap();
        }

        self
    }

    pub fn get_filename_from_block_id(
        &self,
        block_id: &str,
        number_to_name: &BTreeMap<usize, String>,
    ) -> Option<String> {
        let conn = &self.conn;
        //        println!("{}", block_id);
        //        let converted_block_id = base64_url_to_plain(block_id);
        let mut stmt = conn
            .prepare("select FileNum from BlockidToFile Where BlockId = ?")
            .unwrap();
        let mut rows = stmt.query(&[&block_id]).unwrap();
        let row = rows.next();
        if let Ok(row) = row {
            if let Some(row) = row {
                let blocknum: i64 = row.get(0).unwrap();
                Some(
                    number_to_name
                        .get(&(blocknum as usize))
                        .unwrap()
                        .to_string(),
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_content_block(
        &self,
        block_id: &str,
        number_to_name: &BTreeMap<usize, String>,
    ) -> Option<Vec<u8>> {
        let mut output = Vec::new();
        if let Some(filename) = self.get_filename_from_block_id(block_id, number_to_name) {
            let mut zip = zip::ZipArchive::new(File::open(filename).unwrap()).unwrap();
            let mut block = zip.by_name(&base64_plain_to_url(block_id)).unwrap();
            block.read_to_end(&mut output).unwrap();

            Some(output)
        } else {
            None
        }
    }

    pub fn block_size(&self) -> usize {
        self.manifest.block_size as usize
    }

    pub fn offset_size(&self) -> usize {
        // opts['hashes-per-block'] * opts['blocksize']
        let hashes_per_block = self.manifest.block_size / 32; // Assumes SHA-256
        (hashes_per_block * self.manifest.block_size) as usize
    }

    pub fn hash_size(&self) -> usize {
        32
    }
}
