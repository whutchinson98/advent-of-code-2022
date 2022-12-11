use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut contents: Vec<&str> = contents.split("\n").collect();

    contents.pop();

    println!("Part 1 {}", part_one(&contents));
    println!("Part 2 {}", part_two(&contents));
}

fn part_one(content: &Vec<&str>) -> u64 {
    let mut monkeys: Vec<Monkey> = content
        .chunks(7)
        .map(|monkey_lines| {
            let operation_line: Vec<&str> =
                monkey_lines[2].trim_start().trim_end().split(" ").collect();
            let operation: Operation = match operation_line[4] {
                "+" => Operation::Add(operation_line.last().unwrap().to_string()),
                "*" => Operation::Multiply(operation_line.last().unwrap().to_string()),
                _ => {
                    panic!("Invalid operation {}", operation_line[4]);
                }
            };

            let mut items_line: Vec<&str> =
                monkey_lines[1].trim_start().trim_end().split(" ").collect();

            let mut items: Vec<u64> = Vec::new();

            let mut num = items_line.pop().unwrap().to_string();
            while !num.contains("items") {
                if num.contains(",") {
                    num.pop();
                }

                items.push(num.parse::<u64>().unwrap());
                num = items_line.pop().unwrap().to_string();
            }

            let test_condition: u64 = monkey_lines[3]
                .trim_start()
                .trim_end()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            let test_true: u64 = monkey_lines[4]
                .trim_start()
                .trim_end()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            let test_false: u64 = monkey_lines[5]
                .trim_start()
                .trim_end()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            let test = Test {
                true_path: test_true,
                false_path: test_false,
                condition: test_condition,
            };

            return Monkey {
                items: items.into_iter().rev().collect::<Vec<u64>>(),
                operation,
                test,
            };
        })
        .collect();

    let mut inspect_counts: Vec<u64> = vec![0; monkeys.len()];

    // Go for 20 rounds
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];

            inspect_counts[i] = inspect_counts[i] + monkey.items.len() as u64;

            // Operation
            monkey.items = monkey
                .items
                .iter()
                .map(|v| match &monkey.operation {
                    Operation::Add(x) => {
                        if x == "old" {
                            return v * 2;
                        }
                        return x.parse::<u64>().unwrap() + v;
                    }
                    Operation::Multiply(x) => {
                        if x == "old" {
                            return v.pow(2);
                        }
                        return v * x.parse::<u64>().unwrap();
                    }
                })
                .collect();

            // Relief
            monkey.items = monkey.items.iter().map(|v| v / 3).collect();

            let copy_monkey_items = monkey.items.clone();

            // Test
            let test_result: Vec<u64> = monkey
                .items
                .iter()
                .map(|v| {
                    if v % monkey.test.condition == 0 {
                        return monkey.test.true_path;
                    }
                    return monkey.test.false_path;
                })
                .collect();

            monkey.items = Vec::new();
            for j in 0..test_result.len() {
                let catch_monkey = monkeys.get_mut(test_result[j] as usize).unwrap();
                catch_monkey.items.push(copy_monkey_items[j]);
            }
        }
    }

    inspect_counts.sort();
    return inspect_counts[inspect_counts.len() - 1] * inspect_counts[inspect_counts.len() - 2];
}

fn part_two(content: &Vec<&str>) -> u64 {
    let mut monkeys: Vec<Monkey> = content
        .chunks(7)
        .map(|monkey_lines| {
            let operation_line: Vec<&str> =
                monkey_lines[2].trim_start().trim_end().split(" ").collect();
            let operation: Operation = match operation_line[4] {
                "+" => Operation::Add(operation_line.last().unwrap().to_string()),
                "*" => Operation::Multiply(operation_line.last().unwrap().to_string()),
                _ => {
                    panic!("Invalid operation {}", operation_line[4]);
                }
            };

            let mut items_line: Vec<&str> =
                monkey_lines[1].trim_start().trim_end().split(" ").collect();

            let mut items: Vec<u64> = Vec::new();

            let mut num = items_line.pop().unwrap().to_string();
            while !num.contains("items") {
                if num.contains(",") {
                    num.pop();
                }

                items.push(num.parse::<u64>().unwrap());
                num = items_line.pop().unwrap().to_string();
            }

            let test_condition: u64 = monkey_lines[3]
                .trim_start()
                .trim_end()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            let test_true: u64 = monkey_lines[4]
                .trim_start()
                .trim_end()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            let test_false: u64 = monkey_lines[5]
                .trim_start()
                .trim_end()
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            let test = Test {
                true_path: test_true,
                false_path: test_false,
                condition: test_condition,
            };

            return Monkey {
                items: items.into_iter().rev().collect::<Vec<u64>>(),
                operation,
                test,
            };
        })
        .collect();

    let mut inspect_counts: Vec<u64> = vec![0; monkeys.len()];

    // Special thank you to ChristopherBiscardi for this idea
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.condition)
        .product::<u64>();

    // Go for 10000 rounds
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];

            inspect_counts[i] = inspect_counts[i] + monkey.items.len() as u64;

            // Operation
            monkey.items = monkey
                .items
                .iter()
                .map(|v| match &monkey.operation {
                    Operation::Add(x) => {
                        if x == "old" {
                            return (v * 2) % magic_trick;
                        }
                        return (v + x.parse::<u64>().unwrap()) % magic_trick;
                    }
                    Operation::Multiply(x) => {
                        if x == "old" {
                            return v.pow(2) % magic_trick;
                        }
                        return (v * x.parse::<u64>().unwrap()) % magic_trick;
                    }
                })
                .collect();

            // // Relief
            // monkey.items = monkey.items.iter().map(|v| v / 3).collect();

            let copy_monkey_items = monkey.items.clone();

            // Test
            let test_result: Vec<u64> = monkey
                .items
                .iter()
                .map(|v| {
                    if v % monkey.test.condition == 0 {
                        return monkey.test.true_path;
                    }
                    return monkey.test.false_path;
                })
                .collect();

            monkey.items = Vec::new();
            for j in 0..test_result.len() {
                let catch_monkey = monkeys.get_mut(test_result[j] as usize).unwrap();
                catch_monkey.items.push(copy_monkey_items[j]);
            }
        }
    }

    inspect_counts.sort();
    return inspect_counts[inspect_counts.len() - 1] * inspect_counts[inspect_counts.len() - 2];
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Clone, Debug)]
enum Operation {
    Multiply(String),
    Add(String),
}

#[derive(Clone, Copy, Debug)]
struct Test {
    condition: u64,
    true_path: u64,
    false_path: u64,
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::{part_one, part_two};

    #[test]
    #[ignore]
    fn part_one_test() {
        let contents =
            fs::read_to_string("./test.txt").expect("Should have been able to read the file");

        let mut contents: Vec<&str> = contents.split("\n").collect();

        contents.pop();

        assert_eq!(part_one(&contents), 10605);
    }

    #[test]
    fn part_two_test() {
        let contents =
            fs::read_to_string("./test.txt").expect("Should have been able to read the file");

        let mut contents: Vec<&str> = contents.split("\n").collect();

        contents.pop();

        assert_eq!(part_two(&contents), 2713310158);
    }
}
