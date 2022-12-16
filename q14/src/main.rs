use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut contents: Vec<&str> = contents.split("\n").collect();

    contents.pop();

    println!("Part 1 {}", part_one(&contents));
    println!("Part 2 {}", part_two(&contents));
}

enum CaveValue {
    Rock,
    Air,
    Sand,
}

fn part_one(content: &Vec<&str>) -> usize {
    let sand_start = (500, 0);
    let mut current_sand = sand_start;

    let mut board: HashMap<String, CaveValue> = HashMap::new();
    for i in 0..content.len() {
        let line: &str = content[i];
        let val: Vec<bool> = line
            .split(" -> ")
            .collect::<Vec<&str>>()
            .windows(2)
            .map(|chunk| {
                let (x_one_str, y_one_str): (&str, &str) = chunk[0].split_once(",").unwrap();
                let (x_two_str, y_two_str): (&str, &str) = chunk[1].split_once(",").unwrap();

                let (x_one, y_one): (i32, i32) = (
                    x_one_str.parse::<i32>().unwrap(),
                    y_one_str.parse::<i32>().unwrap(),
                );

                let (x_two, y_two): (i32, i32) = (
                    x_two_str.parse::<i32>().unwrap(),
                    y_two_str.parse::<i32>().unwrap(),
                );

                board.insert(format!("{x},{y}", x = x_one, y = y_one), CaveValue::Rock);
                board.insert(format!("{x},{y}", x = x_two, y = y_two), CaveValue::Rock);

                let is_horizontal = x_one - x_two != 0;

                if is_horizontal {
                    let is_backwards = x_one > x_two;
                    let step;

                    match is_backwards {
                        true => step = -1,
                        false => step = 1,
                    };

                    let mut val = x_one + step;
                    loop {
                        board.insert(format!("{val},{y}", val = val, y = y_one), CaveValue::Rock);
                        if val == x_two {
                            break;
                        }
                        val += step;
                    }
                    board.insert(format!("{val},{y}", val = val, y = y_one), CaveValue::Rock);
                } else {
                    let is_backwards = y_one > y_two;
                    let step;

                    match is_backwards {
                        true => step = -1,
                        false => step = 1,
                    };

                    let mut val = y_one + step;
                    loop {
                        board.insert(format!("{x},{val}", x = x_one, val = val), CaveValue::Rock);
                        if val == y_two {
                            break;
                        }
                        val += step;
                    }
                    board.insert(format!("{x},{val}", x = x_one, val = val), CaveValue::Rock);
                }
                return true;
            })
            .collect::<Vec<bool>>();
        println!("lines {}", val.len());
    }

    let rock_count = board.len();

    let mut rocks_vec: Vec<(i32, i32)> = board
        .keys()
        .into_iter()
        .map(|key| {
            let (x_str, y_str): (&str, &str) = key.split_once(",").unwrap();

            let (x, y): (i32, i32) = (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap());

            return (x, y);
        })
        .collect();

    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let lowest_rock = rocks_vec.last().unwrap();

    loop {
        if current_sand.1 > lowest_rock.1 {
            break;
        }
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right = (current_sand.0 + 1, current_sand.1 + 1);
        match (
            board.get(&format!("{x},{y}", x = down.0, y = down.1)),
            board.get(&format!("{x},{y}", x = left.0, y = left.1)),
            board.get(&format!("{x},{y}", x = right.0, y = right.1)),
        ) {
            (Some(_), Some(_), Some(_)) => {
                // no valid move
                // aka "frozen"
                if current_sand == (500, 0) {
                    break;
                }

                board.insert(
                    format!("{x},{y}", x = current_sand.0, y = current_sand.1),
                    CaveValue::Sand,
                );

                current_sand = (500, 0);
            }
            (None, _, _) => {
                // valid down move
                current_sand = down;
            }
            (_, None, _) => {
                //valid left move
                current_sand = left;
            }
            (_, _, None) => {
                // valid right move
                current_sand = right;
            }
        };
    }

    return board.len() - rock_count;
}

fn part_two(content: &Vec<&str>) -> usize {
    let sand_start = (500, 0);
    let mut current_sand = sand_start;

    let mut board: HashMap<String, CaveValue> = HashMap::new();
    for i in 0..content.len() {
        let line: &str = content[i];
        let val: Vec<bool> = line
            .split(" -> ")
            .collect::<Vec<&str>>()
            .windows(2)
            .map(|chunk| {
                let (x_one_str, y_one_str): (&str, &str) = chunk[0].split_once(",").unwrap();
                let (x_two_str, y_two_str): (&str, &str) = chunk[1].split_once(",").unwrap();

                let (x_one, y_one): (i32, i32) = (
                    x_one_str.parse::<i32>().unwrap(),
                    y_one_str.parse::<i32>().unwrap(),
                );

                let (x_two, y_two): (i32, i32) = (
                    x_two_str.parse::<i32>().unwrap(),
                    y_two_str.parse::<i32>().unwrap(),
                );

                board.insert(format!("{x},{y}", x = x_one, y = y_one), CaveValue::Rock);
                board.insert(format!("{x},{y}", x = x_two, y = y_two), CaveValue::Rock);

                let is_horizontal = x_one - x_two != 0;

                if is_horizontal {
                    let is_backwards = x_one > x_two;
                    let step;

                    match is_backwards {
                        true => step = -1,
                        false => step = 1,
                    };

                    let mut val = x_one + step;
                    loop {
                        board.insert(format!("{val},{y}", val = val, y = y_one), CaveValue::Rock);
                        if val == x_two {
                            break;
                        }
                        val += step;
                    }
                    board.insert(format!("{val},{y}", val = val, y = y_one), CaveValue::Rock);
                } else {
                    let is_backwards = y_one > y_two;
                    let step;

                    match is_backwards {
                        true => step = -1,
                        false => step = 1,
                    };

                    let mut val = y_one + step;
                    loop {
                        board.insert(format!("{x},{val}", x = x_one, val = val), CaveValue::Rock);
                        if val == y_two {
                            break;
                        }
                        val += step;
                    }
                    board.insert(format!("{x},{val}", x = x_one, val = val), CaveValue::Rock);
                }
                return true;
            })
            .collect::<Vec<bool>>();
        println!("lines {}", val.len());
    }

    let rock_count = board.len();

    let mut rocks_vec: Vec<(i32, i32)> = board
        .keys()
        .into_iter()
        .map(|key| {
            let (x_str, y_str): (&str, &str) = key.split_once(",").unwrap();

            let (x, y): (i32, i32) = (x_str.parse::<i32>().unwrap(), y_str.parse::<i32>().unwrap());

            return (x, y);
        })
        .collect();

    rocks_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let lowest_rock = rocks_vec.last().unwrap();

    loop {
        // It's on the floor
        if current_sand.1 >= lowest_rock.1 + 2 {
            break;
        }
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right = (current_sand.0 + 1, current_sand.1 + 1);
        match (
            board
                .get(&format!("{x},{y}", x = down.0, y = down.1))
                .or_else(|| {
                    if down.1 == lowest_rock.1 + 2 {
                        Some(&CaveValue::Rock)
                    } else {
                        None
                    }
                }),
            board
                .get(&format!("{x},{y}", x = left.0, y = left.1))
                .or_else(|| {
                    if left.1 == lowest_rock.1 + 2 {
                        Some(&CaveValue::Rock)
                    } else {
                        None
                    }
                }),
            board
                .get(&format!("{x},{y}", x = right.0, y = right.1))
                .or_else(|| {
                    if right.1 == lowest_rock.1 + 2 {
                        Some(&CaveValue::Rock)
                    } else {
                        None
                    }
                }),
        ) {
            (Some(_), Some(_), Some(_)) => {
                // no valid move
                // aka "frozen"
                if current_sand == (500, 0) {
                    break;
                }

                board.insert(
                    format!("{x},{y}", x = current_sand.0, y = current_sand.1),
                    CaveValue::Sand,
                );

                current_sand = (500, 0);
            }
            (None, _, _) => {
                // valid down move
                current_sand = down;
            }
            (_, None, _) => {
                //valid left move
                current_sand = left;
            }
            (_, _, None) => {
                // valid right move
                current_sand = right;
            }
        };
    }

    return board.len() - rock_count + 1;
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let contents = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let contents: Vec<&str> = contents.split("\n").collect();

        assert_eq!(part_one(&contents), 24);
    }

    #[test]
    fn part_two_test() {
        let contents = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let contents: Vec<&str> = contents.split("\n").collect();

        assert_eq!(part_two(&contents), 93);
    }
}
