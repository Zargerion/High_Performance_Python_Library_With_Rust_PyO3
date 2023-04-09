use std::fs;

pub fn is_dir_empty(dir: String) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        return entries.count() == 0;
    }
    false
}