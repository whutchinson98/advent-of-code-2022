use std::collections::HashMap;
use std::fs;

type FileSystem = HashMap<String, usize>;

const DISK_SIZE: i32 = 70_000_000;
const SPACE_NEEDED: i32 = 30_000_000;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let contents = contents.trim();

    let file_system = construct_file_system(contents);

    println!("Part 1 {}", part_one(&file_system));
    println!("Part 2 {}", part_two(&file_system));
}

// Sum the directories <= 100000
fn part_one(file_system: &FileSystem) -> usize {
    return file_system.values().filter(|&sum| *sum <= 100_000).sum();
}

// Delete the smallest directory freeing up enough space for the update
fn part_two(file_system: &FileSystem) -> usize {
    let needed = (SPACE_NEEDED - DISK_SIZE + file_system["/"] as i32) as usize;

    let mut min = DISK_SIZE as usize;

    for disk_size in file_system.values() {
        if disk_size >= &needed && min > *disk_size {
            min = *disk_size;
        }
    }

    return min;
}

fn construct_file_system(contents: &str) -> FileSystem {
    // Initialize current_directory to be /
    let mut current_directory: Vec<String> = vec!["/".to_string()];

    // Each directory with total filesize
    let mut directories = HashMap::new();

    contents
        .split("$ ")
        .skip(1)
        .map(|part| {
            let (expr, body) = part.split_at(2);
            (expr, body.trim())
        })
        .for_each(|cmd| {
            match cmd {
                ("cd", "..") => {
                    // Go back up 1 dir
                    current_directory.pop();
                }
                ("cd", dirname) => {
                    // Cd into new dir
                    current_directory.push(format!(
                        "{parent_dir}{dirname}/",
                        parent_dir = current_directory.last().unwrap(),
                        dirname = dirname
                    ));
                }
                ("ls", body) => {
                    // List all files and dirs under working directory
                    // We only care about the filesizes here since we will visit the directories
                    // through the above matches
                    body.split("\n")
                        .map(|l| l.split(" ").next().unwrap())
                        .filter_map(|filesize| filesize.parse::<usize>().ok())
                        .for_each(|filesize| {
                            current_directory.iter().for_each(|dir| {
                                directories
                                    .entry(dir.clone()) // Needs to be cloned for the HashMap key
                                    // to be a String
                                    .and_modify(|size| *size += filesize)
                                    .or_insert(filesize);
                            })
                        });
                }
                _ => panic!("Invalid"),
            };
        });

    return directories;
}
