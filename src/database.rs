use crate::blockid::*;
use rusqlite::types::ToSql;
use rusqlite::*;
use std::collections::BTreeMap;

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new(file: &str) -> DB {
        let conn = Connection::open(file).unwrap();
        DB { conn }
    }

    pub fn create_block_id_to_filenames(self, number_to_name: &BTreeMap<usize, String>) -> Self {
        let conn = &self.conn;

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

        self
    }
}
