use std::fs;

fn main() {
    println!("In file {}", "input.txt");

    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let rounds: Vec<&str> = contents.split("\n").collect();

    println!("Part 1 {}", part_one(&rounds));
    println!("Part 2 {}", part_two(&rounds));
}

fn part_one(rounds: &Vec<&str>) -> i32 {
    let results: Vec<i32> = rounds
        .iter()
        .map(|&val| {
            let game: Vec<&str> = val.split(" ").collect();
            if game[0] == "" {
                return 0;
            }
            return get_score(&game);
        })
        .collect();
    return results.iter().sum();
}

fn part_two(rounds: &Vec<&str>) -> i32 {
    let results: Vec<i32> = rounds
        .iter()
        .map(|&val| {
            let game: Vec<&str> = val.split(" ").collect();
            if game[0] == "" {
                return 0;
            }
            return get_score_two(&game);
        })
        .collect();
    return results.iter().sum();
}

fn get_score(game: &Vec<&str>) -> i32 {
    let enemy;
    let you;

    if game[0] == "A" {
        enemy = "R"
    } else if game[0] == "B" {
        enemy = "P"
    } else if game[0] == "C" {
        enemy = "S"
    } else {
        panic!("Enemy not valid value {}", game[0])
    }

    if game[1] == "X" {
        you = "R"
    } else if game[1] == "Y" {
        you = "P"
    } else if game[1] == "Z" {
        you = "S"
    } else {
        panic!("You not valid value {}", game[1])
    }

    return winner_score(you, enemy) + item_score(you);
}

fn winner_score(you: &str, enemy: &str) -> i32 {
    // Draw
    if enemy == you {
        return 3;
    }

    // enemy wins r beats s, s beats p, p beats r
    if enemy == "R" && you == "S" || enemy == "S" && you == "P" || enemy == "P" && you == "R" {
        return 0;
    }

    // you win
    return 6;
}

fn item_score(item: &str) -> i32 {
    if item == "R" {
        return 1;
    }
    if item == "P" {
        return 2;
    }
    if item == "S" {
        return 3;
    }
    return -1;
}

fn get_score_two(game: &Vec<&str>) -> i32 {
    let enemy;
    let fixed_result;
    let you;

    if game[0] == "A" {
        enemy = "R";
    } else if game[0] == "B" {
        enemy = "P";
    } else if game[0] == "C" {
        enemy = "S";
    } else {
        panic!("Enemy not valid value {}", game[0])
    }

    if game[1] == "X" {
        fixed_result = "L";
    } else if game[1] == "Y" {
        fixed_result = "D";
    } else if game[1] == "Z" {
        fixed_result = "W";
    } else {
        panic!("You not valid value {}", game[1])
    }

    // Win
    if fixed_result == "W" {
        if enemy == "R" {
            you = "P";
        } else if enemy == "P" {
            you = "S";
        } else {
            you = "R";
        }
        return 6 + item_score(you);
    } else if fixed_result == "L" {
        // Lose
        if enemy == "R" {
            you = "S";
        } else if enemy == "P" {
            you = "R";
        } else {
            you = "P";
        }
        return item_score(you);
    }

    // Draw
    return 3 + item_score(enemy);
}
