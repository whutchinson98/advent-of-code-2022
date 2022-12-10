use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut contents: Vec<&str> = contents.split("\n").collect();

    contents.pop();

    println!("Part 1 {}", part_one(&contents));

    println!("Part 2:");
    println!("{}", part_two(&contents));
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }
}

fn part_one(content: &Vec<&str>) -> i32 {
    let important_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut cycle_values: HashMap<u32, i32> = HashMap::new();

    let instructions: Vec<Instruction> = content
        .iter()
        .map(|val| {
            let line: Vec<&str> = val.split(" ").collect();
            match line[0] {
                "addx" => Instruction::Add(line[1].parse::<i32>().unwrap()),
                "noop" => Instruction::Noop,
                _ => panic!("Unknown match {:?}", line),
            }
        })
        .collect();

    let mut cycle = 0;
    let mut x: i32 = 1;
    for instruction in instructions {
        // Incorrect values might be inserted for example at cycle 18 if it is noop
        // But when the cycle is correctly going to be 20 the correct value will update the hashmap
        if important_cycles.contains(&(cycle + 1)) {
            cycle_values.insert(cycle + 1, (cycle as i32 + 1) * x);
        } else if important_cycles.contains(&(cycle + 2)) {
            cycle_values.insert(cycle + 2, (cycle as i32 + 2) * x);
        }

        cycle += instruction.cycles();

        match instruction {
            Instruction::Noop => {}
            Instruction::Add(val) => {
                x += val;
            }
        }
    }

    return cycle_values.values().sum::<i32>();
}

fn part_two(content: &Vec<&str>) -> String {
    let mut crt_pixels: String = "".to_string();

    let instructions: Vec<Instruction> = content
        .iter()
        .map(|val| {
            let line: Vec<&str> = val.split(" ").collect();
            match line[0] {
                "addx" => Instruction::Add(line[1].parse::<i32>().unwrap()),
                "noop" => Instruction::Noop,
                _ => panic!("Unknown match {:?}", line),
            }
        })
        .collect();

    let mut cycle: u32 = 0;
    let mut x: i32 = 1;
    for instruction in instructions {
        for num_cycles in 0..instruction.cycles() {
            crt_pixels.push_str(&draw_pixel(x, ((cycle + num_cycles) % 40) as i32));
        }

        cycle += instruction.cycles();

        match instruction {
            Instruction::Noop => {}
            Instruction::Add(val) => {
                x += val;
            }
        }
    }

    return crt_pixels
        .chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn draw_pixel(x: i32, pixel: i32) -> String {
    // X must be within +/- 1 of the given pixel to be drawn
    if x - 1 == pixel || x == pixel || x + 1 == pixel {
        return "#".to_string();
    } else {
        return ".".to_string();
    }
}
#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    #[ignore]
    fn part_one_test() {
        let contents = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let contents: Vec<&str> = contents.split("\n").collect();

        assert_eq!(part_one(&contents), 13140);
    }

    #[test]
    fn part_two_test() {
        let contents = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let contents: Vec<&str> = contents.split("\n").collect();

        assert_eq!(
            part_two(&contents),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
