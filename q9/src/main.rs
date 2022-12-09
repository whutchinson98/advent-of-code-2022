use std::collections::HashSet;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut contents: Vec<&str> = contents.split("\n").collect();

    contents.pop();

    println!("Part 1 {}", part_one(&contents));
    println!("Part 2 {}", part_two(&contents));
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    direction: Direction,
    amount: i32,
}

fn direction(input: &str) -> Direction {
    match input {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Unknown option"),
    }
}

fn part_one(content: &Vec<&str>) -> usize {
    let instructions: Vec<Move> = content
        .iter()
        .map(|v| {
            let instruction: Vec<&str> = v.split(" ").collect();

            return Move {
                direction: direction(instruction[0]),
                amount: instruction[1].parse::<i32>().unwrap(),
            };
        })
        .collect();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut seen_positions = HashSet::from([tail]);

    for instruction in instructions {
        // For each instruction execute it. update head and then check if tail needs to move
        for _ in 0..instruction.amount {
            // Update head position
            match instruction.direction {
                Direction::Left => {
                    head.0 -= 1;
                }
                Direction::Right => {
                    head.0 += 1;
                }
                Direction::Up => {
                    head.1 += 1;
                }
                Direction::Down => {
                    head.1 -= 1;
                }
            }

            // Check if tail needs to be updated
            if !is_knot_connected(&head, &tail) {
                // Copy head position and move tail to 1 beside the head based on direction of move
                let mut new_tail = head.clone();
                match instruction.direction {
                    Direction::Left => {
                        new_tail.0 += 1;
                    }
                    Direction::Right => {
                        new_tail.0 -= 1;
                    }
                    Direction::Up => {
                        new_tail.1 -= 1;
                    }
                    Direction::Down => {
                        new_tail.1 += 1;
                    }
                }
                tail = new_tail;
                seen_positions.insert(new_tail);
            }
        }
    }

    return seen_positions.len();
}

fn is_knot_connected(parent: &(i32, i32), child: &(i32, i32)) -> bool {
    // xxx
    // xHx
    // xxx

    // above
    if parent.1 + 1 == child.1
        && (parent.0 - 1 == child.0 || parent.0 == child.0 || parent.0 + 1 == child.0)
    {
        // child is above the parent
        return true;
    }

    // right
    if parent.0 + 1 == child.0 && parent.1 == child.1 {
        // child is right of parent
        return true;
    }

    // below
    if parent.1 - 1 == child.1
        && (parent.0 - 1 == child.0 || parent.0 == child.0 || parent.0 + 1 == child.0)
    {
        // child is below the parent
        return true;
    }

    // left
    if parent.0 - 1 == child.0 && parent.1 == child.1 {
        // child is left the parent
        return true;
    }

    // on top of
    if parent.0 == child.0 && parent.1 == child.1 {
        return true;
    }

    // child needs to move
    return false;
}

fn part_two(content: &Vec<&str>) -> usize {
    let instructions: Vec<Move> = content
        .iter()
        .map(|v| {
            let instruction: Vec<&str> = v.split(" ").collect();

            return Move {
                direction: direction(instruction[0]),
                amount: instruction[1].parse::<i32>().unwrap(),
            };
        })
        .collect();

    let mut ropes = [(0, 0); 10]; // 10 knots in the same position
    let mut seen_positions = HashSet::from([ropes[ropes.len() - 1]]);

    for instruction in instructions {
        // For each instruction execute it. update head and then check if tail needs to move
        for _ in 0..instruction.amount {
            // Update head position
            match instruction.direction {
                Direction::Left => {
                    ropes[0].0 -= 1;
                }
                Direction::Right => {
                    ropes[0].0 += 1;
                }
                Direction::Up => {
                    ropes[0].1 += 1;
                }
                Direction::Down => {
                    ropes[0].1 -= 1;
                }
            }

            for next in 1..10 {
                let head = next - 1;

                if !is_knot_connected(&ropes[head], &ropes[next]) {
                    // Get the difference of the 2 positions
                    let diff = (ropes[head].0 - ropes[next].0, ropes[head].1 - ropes[next].1);

                    // We will need to move the next knot based on the difference
                    if diff.0 > 0 {
                        ropes[next].0 += 1;
                    }

                    if diff.0 < 0 {
                        ropes[next].0 -= 1;
                    }

                    if diff.1 > 0 {
                        ropes[next].1 += 1;
                    }

                    if diff.1 < 0 {
                        ropes[next].1 -= 1;
                    }

                    if next == 9 {
                        seen_positions.insert(ropes[9]);
                    }
                }
            }
        }
    }

    return seen_positions.len();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let contents =
            fs::read_to_string("./test.txt").expect("Should have been able to read the file");

        let mut contents: Vec<&str> = contents.split("\n").collect();

        contents.pop();

        assert_eq!(part_one(&contents), 13);
    }

    #[test]
    fn part_two_test() {
        let contents =
            fs::read_to_string("./test2.txt").expect("Should have been able to read the file");

        let mut contents: Vec<&str> = contents.split("\n").collect();

        contents.pop();

        assert_eq!(part_two(&contents), 36);
    }
}
