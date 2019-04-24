mod blockid;
mod database;

use blockid::*;
use database::*;
use num_cpus;
use pbr::ProgressBar;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;
use std::sync::{Arc, Mutex};
use zip;

fn main() {
    println!("Enter the location of the backup:");
    let mut backup_dir = String::new();
    stdin()
        .read_line(&mut backup_dir)
        .expect("Did not enter a location.");
    println!();
    let backup_dir = backup_dir.trim();

    println!("Enter a location to restore to:");
    let mut restore_dir = String::new();
    stdin()
        .read_line(&mut restore_dir)
        .expect("Did not enter a location.");
    println!();
    let restore_dir = restore_dir.trim();

    let db_location = Path::join(Path::new(backup_dir), Path::new("index.db"));
    let db_location = db_location.to_str().unwrap();

    println!(
        "Enter number of threads to use (Default {}):",
        num_cpus::get()
    );
    let mut cpu_input = String::new();
    stdin()
        .read_line(&mut cpu_input)
        .expect("Did not enter a number");
    let cpu_count: usize = match cpu_input.trim().parse() {
        Ok(i) => i,
        Err(..) => num_cpus::get(),
    };
    println!();

    // Set CPU count
    rayon::ThreadPoolBuilder::new()
        .num_threads(cpu_count)
        .build_global()
        .unwrap();

    // Find newest dlist
    let mut dlist_file_names: Vec<String> = fs::read_dir(&backup_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|f| f.path().to_str().unwrap().ends_with("dlist.zip"))
        .map(|f| f.path().to_str().unwrap().to_string())
        .collect();

    dlist_file_names.sort();

    let dlist = dlist_file_names[dlist_file_names.len() - 1].clone();

    println!("{} appears to be newest dlist, using it.", dlist);
    println!("Parsing dlist");

    // Open dlist file
    let mut dlist_zip = zip::ZipArchive::new(File::open(dlist.clone()).unwrap()).unwrap();
    let mut dlist_file = dlist_zip.by_name("filelist.json").unwrap();
    let mut dlist_contents = String::new();
    dlist_file.read_to_string(&mut dlist_contents).unwrap();
    let file_entries = parse_dlist(&dlist_contents);

    // Open Manifest
    let mut manifest_zip = zip::ZipArchive::new(File::open(dlist.clone()).unwrap()).unwrap();
    let mut manifest_file = manifest_zip.by_name("manifest").unwrap();
    let mut manifest_contents = String::new();
    manifest_file
        .read_to_string(&mut manifest_contents)
        .unwrap();
    let manifest_contents = manifest_contents.replace("\u{feff}", "");
    let manifest_contents = manifest_contents.trim();

    let file_count = file_entries.iter().filter(|f| f.is_file()).count();
    println!("{} files to be restored", file_count);
    let folder_count = file_entries.iter().filter(|f| f.is_folder()).count();
    println!("{} folders to be restored", folder_count);
    println!();

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
    let dblock_db =
        DB::new(db_location, &manifest_contents).create_block_id_to_filenames(&number_to_name);

    println!("Restoring directory structure");
    let mut pb = ProgressBar::new(folder_count as u64);
    for d in file_entries.iter().filter(|f| f.is_folder()) {
        d.restore_file(&dblock_db, &number_to_name, &restore_dir);
        pb.inc();
    }
    println!();

    println!("Restoring files");
    let pb = Arc::new(Mutex::new(ProgressBar::new(file_count as u64)));
    file_entries
        .par_iter()
        .filter(|f| f.is_file())
        .for_each(|f| {
            f.restore_file(&dblock_db, &number_to_name, &restore_dir);
            pb.lock().unwrap().inc();
        });
}
