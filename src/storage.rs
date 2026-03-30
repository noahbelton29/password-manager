use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::entry::Entry;

pub fn save_entry(entry: &Entry) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("passwords.txt")
        .unwrap();

    let line = entry.to_line() + "\n";
    file.write_all(line.as_bytes()).unwrap();
}

pub fn load_entries() -> Vec<Entry> {
    let contents = fs::read_to_string("passwords.txt").unwrap_or_default();
    contents.lines().filter_map(Entry::from_line).collect()
}

pub fn overwrite_entries(entries: &[Entry]) {
    let contents: String = entries
        .iter()
        .map(|e| e.to_line())
        .collect::<Vec<String>>()
        .join("\n") + "\n";

    fs::write("passwords.txt", contents).unwrap();
}

pub fn delete_entry_by_index(index: usize) {
    let mut entries = load_entries();
    if index >= entries.len() {
        eprintln!("Invalid index");
        return;
    }
    entries.remove(index);
    overwrite_entries(&entries);
}