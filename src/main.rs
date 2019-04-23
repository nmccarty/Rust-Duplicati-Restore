mod blockid;
mod database;

use std::collections::BTreeMap;
use std::fs;

use database::*;

fn main() {
    let backup_dir = "/home/nmccarty/tmp/config/";
    let db_location = "/home/nmccarty/tmp/config/index.db";
    let restore_dir = "/home/nmccarty/tmp/restore";

    // Get list of dblocks
    let zip_file_names = fs::read_dir(backup_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|f| f.path().to_str().unwrap().ends_with("dblock.zip"));

    // Assign each dblock file a number
    let mut number_to_name = BTreeMap::new();
    for (i, file) in zip_file_names.enumerate() {
        number_to_name.insert(i, file.path().display().to_string());
    }

    println!("Found {} dblocks", number_to_name.len());

    // Open dblock db connection and build db
    println!();
    println!("Indexing dblocks");
    let dblock_db = DB::new(db_location).create_block_id_to_filenames(&number_to_name);

    // Find newest dlist
    let mut dlist_file_names: Vec<String> = fs::read_dir(backup_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|f| f.path().to_str().unwrap().ends_with("dlist.zip"))
        .map(|f| f.path().file_name().unwrap().to_str().unwrap().to_string())
        .collect();

    dlist_file_names.sort();

    let dlist = dlist_file_names[dlist_file_names.len() - 1].clone();

    println!("{} appears to be newest dlist, using it.", dlist);
}
