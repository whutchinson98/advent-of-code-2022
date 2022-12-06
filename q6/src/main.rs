use std::collections::HashMap;
use std::fs;

fn main() {
    let contents: String =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    println!("Part 1 {}", part_one(&contents));

    println!("Part 2 {}", part_two(&contents));
}

fn part_one(message: &str) -> usize {
    let mut subroutine: Vec<char> = Vec::new();

    for i in 0..message.len() {
        // remove item from the start of the vec before adding the new item
        if subroutine.len() == 4 {
            subroutine.remove(0);
        }
        let val: char = message.chars().nth(i).unwrap();

        subroutine.push(val);

        if is_subroutine(&subroutine) && subroutine.len() == 4 {
            return i + 1;
        }
    }
    return 0;
}

fn is_subroutine(part: &Vec<char>) -> bool {
    let mut seen: HashMap<char, bool> = HashMap::new();
    for val in part {
        if seen.contains_key(&val) {
            return false;
        }
        seen.insert(*val, true);
    }
    return true;
}

fn part_two(message: &str) -> usize {
    let mut subroutine: Vec<char> = Vec::new();

    for i in 0..message.len() {
        if subroutine.len() == 14 {
            subroutine.remove(0);
        }
        let val: char = message.chars().nth(i).unwrap();

        subroutine.push(val);

        if is_subroutine(&subroutine) && subroutine.len() == 14 {
            return i + 1;
        }
    }
    return 0;
}
