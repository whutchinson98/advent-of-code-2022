/*
 * Part 1: Split the entry in the middle
 * find the first common repeating character
 * get priority of that character and add to sum
 * ---------------------------------------------
 * Part 2: Every 3 lines is a group of elves
 * Find the common item time between all three of them
 * use that as value to add to sum
 */

use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut entries: Vec<&str> = contents.split("\n").collect();
    entries.pop();

    println!("Part 1 {}", part_one(&entries));
    println!("Part 2 {}", part_two(&entries));
}

fn create_priority_map() -> HashMap<String, usize> {
    let mut priorities_map: HashMap<String, usize> = HashMap::new();

    let priorities: Vec<&str> = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ]
    .to_vec();

    for i in 0..priorities.len() {
        priorities_map.insert(priorities[i].to_string(), i + 1);
    }

    return priorities_map;
}

fn get_first_common_char(val: &str) -> String {
    let (first, second): (&str, &str) = val.split_at(val.len() / 2);

    return first
        .chars()
        .find(|c| second.chars().any(|b| b == *c))
        .unwrap()
        .to_string();
}

fn part_one(entries: &Vec<&str>) -> usize {
    let priority_map: HashMap<String, usize> = create_priority_map();

    let priorities: Vec<usize> = entries
        .iter()
        .map(|&val| {
            let common_char: String = get_first_common_char(val);

            let val = priority_map.get(&common_char);

            return *val.to_owned().unwrap();
        })
        .collect();

    return priorities.iter().sum();
}

fn part_two(entries: &Vec<&str>) -> usize {
    let priority_map: HashMap<String, usize> = create_priority_map();

    let priorities: Vec<usize> = entries
        .chunks(3)
        .map(|elfs| {
            return elfs[0]
                .chars()
                .find(|b| elfs[1].contains(*b) && elfs[2].contains(*b))
                .unwrap();
        })
        .map(|char| *priority_map.get(&char.to_string()).unwrap())
        .collect();

    return priorities.iter().sum();
}
