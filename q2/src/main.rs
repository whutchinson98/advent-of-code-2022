use std::fs;
// The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissorsc
// second column what you play X for Rock, Y for Paper, and Z for Scissors
// SCORE:
// 1 for Rock, 2 for Paper, and 3 for Scissors
// 0 if you lost, 3 if the round was a draw, and 6 if you won
fn main() {
    println!("In file {}", "input.txt");

    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let rounds: Vec<&str> = contents.split("\n").collect();

    // for each round split the string by space and get running score going
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

    println!("Part 1 {}", results.iter().sum::<i32>());
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
