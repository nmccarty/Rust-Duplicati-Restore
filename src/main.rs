mod blockid;
mod database;

use std::collections::BTreeMap;
use std::fs;

fn main() {
    let backup_dir = "/home/nmccarty/tmp/config/";

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
}
