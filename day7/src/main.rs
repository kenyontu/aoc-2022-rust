use regex::Regex;
use std::{collections::HashMap, fs, path::PathBuf};

struct Directory {
    size: u32,
}

impl Directory {
    fn new(size: u32) -> Self {
        Directory { size }
    }
}

/// Maintains the sizes of all directories and what is the current directory
struct FileSystemBuilder {
    path: PathBuf,
    dir_stack: Vec<String>,
    dir_map: HashMap<String, Directory>,
}

impl FileSystemBuilder {
    fn new() -> Self {
        Self {
            path: PathBuf::new(),
            dir_stack: Vec::new(),
            dir_map: HashMap::new(),
        }
    }

    fn go_to_dir(&mut self, dir_name: &str) {
        // Handles the case where the user changes directory using an absolute
        // path
        if dir_name[0..1] == "/".to_string() {
            self.path.clear()
        }

        self.path.push(dir_name);

        let absolute_path = self.path.to_str().unwrap().to_string();
        self.dir_stack.push(absolute_path.clone());

        self.dir_map
            .insert(absolute_path.clone(), Directory::new(0));
    }

    fn go_to_parent_dir(&mut self) {
        self.path.pop();
        let prev_dir_name = self.dir_stack.pop().unwrap();
        let prev_dir = self.dir_map.get(&prev_dir_name).unwrap();

        self.increase_current_dir_size(prev_dir.size);
    }

    fn increase_current_dir_size(&mut self, value: u32) {
        if let Some(dir_name) = self.dir_stack.last() {
            self.dir_map
                .entry(dir_name.to_owned())
                .and_modify(|dir| dir.size += value);
        }
    }

    /// Returns a vector with all directories.
    /// It will consume this [FileSystemBuilder].
    fn to_vector(self) -> Vec<Directory> {
        self.dir_map.into_values().collect()
    }
}

fn parse_input(input: String) -> (u32, Vec<Directory>) {
    // Regex to match change directory commands
    let cd_regex = Regex::new(r"^\$ cd (?<dir>.+)$").unwrap();
    // Regex to match the size of a file from the ls result
    let file_regex = Regex::new(r"^(?<size>\d+) .+$").unwrap();

    let mut fs_builder = FileSystemBuilder::new();

    for line in input.lines() {
        // Handle change directory command
        if let Some(captures) = cd_regex.captures(line) {
            let dir_name = &captures["dir"];

            if dir_name == ".." {
                fs_builder.go_to_parent_dir();
            } else {
                fs_builder.go_to_dir(dir_name);
            }

            continue;
        }

        // Handle line with file size
        if let Some(captures) = file_regex.captures(line) {
            let file_size = captures["size"].parse::<u32>().unwrap();
            fs_builder.increase_current_dir_size(file_size);
            continue;
        }
    }

    // We do this because the input might not return to the root directory at
    // the end, and we need this to so the size of the current directory is added
    // to the parent's
    while fs_builder.dir_stack.len() > 1 {
        fs_builder.go_to_parent_dir();
    }

    // To get the total used space, we can just get the size of the root directory
    let total_used_space = fs_builder.dir_map.get("/").unwrap().size;

    (total_used_space, fs_builder.to_vector())
}

fn main() {
    let file = "input.txt";
    let contents = fs::read_to_string(file).expect(&format!("Error reading the {} file.", file));

    let (total_used_space, directories) = parse_input(contents);

    let total_space = 70_000_000;
    let space_required = (30_000_000 - (total_space - total_used_space) as i32).abs();

    let mut min_space = total_used_space;
    for dir in directories.iter() {
        if dir.size as i32 > space_required && dir.size < min_space {
            min_space = dir.size;
        }
    }

    println!(
        "The folder with the minimum space required is: {}",
        min_space
    );
}
