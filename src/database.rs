use crate::blockid::*;
use pbr::ProgressBar;
use rusqlite::types::ToSql;
use rusqlite::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use zip::*;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new(file: &str) -> DB {
        let conn = Connection::open(file).unwrap();
        conn.execute("PRAGMA temp_store = memory", NO_PARAMS)
            .unwrap();
        conn.execute("PRAGMA page_size = 16384", NO_PARAMS).unwrap();
        conn.execute("PRAGMA cache_size = 2048", NO_PARAMS).unwrap();
        DB { conn }
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
}
