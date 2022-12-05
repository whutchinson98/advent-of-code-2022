use std::fs;
/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
*/

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let contents: Vec<_> = contents.split("\n\n").collect();

    let mut init_cargo: Vec<&str> = contents[0].split("\n").collect();
    let mut entries: Vec<&str> = contents[1].split("\n").collect();

    // Pop last entry as it's ""
    entries.pop();

    // Remove the 1 -> 9 line
    init_cargo.pop();

    let mut cargo: Vec<Vec<char>> = create_stacks(&init_cargo);
    let moves: Vec<Move> = create_moves(&entries);

    println!("Part 1 {}", part_one(&mut cargo, &moves));

    let mut cargo: Vec<Vec<char>> = create_stacks(&init_cargo);
    println!("Part 2 {}", part_two(&mut cargo, &moves));
}

fn create_stacks(entries: &Vec<&str>) -> Vec<Vec<char>> {
    let stack_lines: Vec<Vec<char>> = entries
        .iter()
        .map(|val| {
            let items: Vec<&str> = val.split("").collect();

            let collected_vals: Vec<char> = items
                .chunks(4) // Each value is effectively 4 chars "[A] " OR "    "
                .filter(|v| v.len() > 1) // Remove any end of line entries
                .map(|cargo_entry| {
                    if cargo_entry.join("") == "    " {
                        // blank entry
                        return ' ';
                    }
                    return cargo_entry.get(2).unwrap().parse::<char>().unwrap();
                })
                .collect();

            return collected_vals;
        })
        .collect();

    let mut cargo_stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..9 {
        cargo_stacks.push(Vec::new());
    }

    let mut cargo_count = 0;
    for line in stack_lines {
        for val in line {
            if val != ' ' {
                cargo_stacks[cargo_count].push(val);
            }
            cargo_count += 1;
        }
        cargo_count = 0;
    }

    return cargo_stacks
        .into_iter()
        .map(|x| x.into_iter().rev().collect())
        .collect();
}

fn create_moves(entries: &Vec<&str>) -> Vec<Move> {
    return entries
        .iter()
        .map(|val| {
            let move_array: Vec<&str> = val.split(" ").collect();
            return Move {
                count: move_array[1].parse::<usize>().unwrap(),
                from: move_array[3].parse::<usize>().unwrap(),
                to: move_array[5].parse::<usize>().unwrap(),
            };
        })
        .collect();
}

fn part_one(cargo: &mut Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for m in moves {
        for _i in 0..m.count {
            let val = cargo[m.from - 1].pop();
            cargo[m.to - 1].push(val.unwrap());
        }
    }

    let result: Vec<char> = cargo
        .iter()
        .map(|val| {
            return val[val.len() - 1];
        })
        .collect::<Vec<char>>();

    return result.into_iter().collect();
}

fn part_two(cargo: &mut Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for m in moves {
        let mut to_move: Vec<char> = Vec::new();
        for _i in 0..m.count {
            let val = cargo[m.from - 1].pop();
            to_move.push(val.unwrap());
        }
        let sorted_to_move: Vec<char> = to_move.into_iter().rev().collect();

        for v in sorted_to_move {
            cargo[m.to - 1].push(v);
        }
    }

    let result: Vec<char> = cargo
        .iter()
        .map(|val| {
            return val[val.len() - 1];
        })
        .collect::<Vec<char>>();

    return result.into_iter().collect();
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}
