use serde::{Deserialize, Serialize};
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut entries: Vec<&str> = contents.split("\n").collect();
    println!("{}", entries[entries.len() - 1]);

    // Pop last entry as it's ""
    entries.pop();

    println!("Part 1 {}", part_one(&entries));
    println!("Part 2 {}", part_two(&entries));
}

#[derive(Serialize, Deserialize, Debug)]
struct CleanAssignments {
    start: i32,
    end: i32,
}

fn check_overlap(assignments: &Vec<&str>) -> i32 {
    // split each assignment by - convert to int
    let one: Vec<&str> = assignments[0].split("-").collect();
    let two: Vec<&str> = assignments[1].split("-").collect();

    let assignment_one = CleanAssignments {
        start: one[0].parse::<i32>().unwrap(),
        end: one[1].parse::<i32>().unwrap(),
    };

    let assignment_two = CleanAssignments {
        start: two[0].parse::<i32>().unwrap(),
        end: two[1].parse::<i32>().unwrap(),
    };

    // One engulfs Two
    if assignment_one.start <= assignment_two.start && assignment_one.end >= assignment_two.end {
        return 1;
    }

    // Two engulfs One
    if assignment_two.start <= assignment_one.start && assignment_two.end >= assignment_one.end {
        return 1;
    }

    return 0;
}

// p2 is super similar to p1 aside from the condition of how it overlaps
fn check_overlap_p2(assignments: &Vec<&str>) -> i32 {
    // split each assignment by - convert to int
    let one: Vec<&str> = assignments[0].split("-").collect();
    let two: Vec<&str> = assignments[1].split("-").collect();

    let assignment_one = CleanAssignments {
        start: one[0].parse::<i32>().unwrap(),
        end: one[1].parse::<i32>().unwrap(),
    };

    let assignment_two = CleanAssignments {
        start: two[0].parse::<i32>().unwrap(),
        end: two[1].parse::<i32>().unwrap(),
    };

    if assignment_one.start <= assignment_two.end && assignment_one.end >= assignment_two.start {
        return 1;
    }

    if assignment_two.start <= assignment_one.end && assignment_two.end >= assignment_one.start {
        return 1;
    }

    return 0;
}

fn part_one(entries: &Vec<&str>) -> i32 {
    let map_result: i32 = entries
        .iter()
        .map(|&val| {
            let assignments: Vec<&str> = val.split(",").collect();
            return check_overlap(&assignments);
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    return map_result;
}

fn part_two(entries: &Vec<&str>) -> i32 {
    let map_result: i32 = entries
        .iter()
        .map(|&val| {
            let assignments: Vec<&str> = val.split(",").collect();
            return check_overlap_p2(&assignments);
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    return map_result;
}
