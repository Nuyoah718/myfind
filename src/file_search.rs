use regex::Regex;
use std::fs;
use std::path::Path;

pub fn find_files(root: &str, pattern: &str, command: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let regex = Regex::new(pattern)?;
    let mut matches = Vec::new();
    walk_tree(Path::new(root), &regex, &command, &mut matches)?;
    Ok(matches)
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    command: &str,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, command, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                    if command == "-v" {
                        println!("{} ", filename);
                    }
                }
            }
        }
    }
    Ok(())
}
