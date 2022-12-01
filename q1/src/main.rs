use std::fs;

fn main() {
    println!("In file {}", "input.txt");

    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    // Get the elf calories input
    let elfs_calories_str: Vec<&str> = contents.split("\n\n").collect();

    // Create the vec of elfs and their calories
    let elfs_calories: Vec<Vec<i32>> = elfs_calories_str
        .iter()
        .map(|&val| {
            // Split the string by line ending to get each caloric value
            let elf_calories_str: Vec<&str> = val.split("\n").collect();

            // Go through each caloric value and convert str to i32
            let elf_calories: Vec<i32> = elf_calories_str
                .iter()
                .map(|&val| {
                    // if it fails to unwrap due to the item being "" it defaults to 0
                    return val.parse::<i32>().unwrap_or(0);
                })
                .collect();
            return elf_calories;
        })
        .collect();

    part1(&elfs_calories);
    part2(&elfs_calories);
}

fn part1(elfs_calories: &Vec<Vec<i32>>) {
    let mut max = 0;

    for elf in elfs_calories {
        let mut elf_cal = 0;
        for cal in elf {
            elf_cal += cal;
        }

        if max < elf_cal {
            max = elf_cal
        }
    }
    println!("part 1 {}", max);
}

fn part2(elfs_calories: &Vec<Vec<i32>>) {
    let mut elf_totals: Vec<i32> = elfs_calories
        .iter()
        .map(|elf_cals| {
            let mut total_cals = 0;

            for i in 0..elf_cals.len() {
                total_cals += elf_cals[i];
            }
            return total_cals;
        })
        .collect();

    elf_totals.sort();
    elf_totals.reverse();

    println!("part 2 {}", (elf_totals[0] + elf_totals[1] + elf_totals[2]));
}
