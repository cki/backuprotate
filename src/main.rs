mod path_filter;

use std::fs;
use std::env;
use std::collections::HashSet;

use std::path::{Path, PathBuf};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

use std::iter::FromIterator;

fn get_backup_files(folder_path: &Path, file_prefix: &String) -> Vec<PathBuf> {
    use std::fs::read_dir;
    
    if !folder_path.is_dir() {
        panic!("backupfolder is not a folder");
    }

    read_dir(folder_path)
        .expect("could not read dir")
        .map(|entry| entry.unwrap().path())
        .filter(|path| !path.is_dir())
        .filter(|path| path.file_name()
                .expect("could not get fname")
                .to_str()
                .expect("could not get str")
                .starts_with(file_prefix))
        .collect::<Vec<PathBuf>>()
}

fn main() {
    let file_prefix = env::var("FILE_PREFIX").expect("expected FILE_PREFIX=");
    let backup_env_path = env::var("BACKUP_PATH").expect("expected BACKUP_PATH=");
    let backup_path = Path::new(&backup_env_path);

    let all_possible_backup_files = get_backup_files(backup_path, &file_prefix);
    let files_to_keep = path_filter::filter(&file_prefix, &all_possible_backup_files);
    let allowed_pathes: HashSet<&Path> = HashSet::from_iter(files_to_keep.iter().cloned());

    for path in &all_possible_backup_files {
        if !allowed_pathes.contains(path.as_path()) {
            println!("deleting {:?}", path);
            // fs::remove_file(path);
        }
    }
}
