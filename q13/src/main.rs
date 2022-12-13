use std::cmp::Ordering;
use std::fs;

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    // 10 is the biggest number in the input replace with a single character
    let contents = content.replace("10", "z");

    let contents: Vec<&str> = contents.split("\n\n").collect();

    println!("Part 1 {}", part_one(&contents));

    let part_two_str = content.replace("\n\n", "\n").replace("10", "z");
    let mut contents: Vec<&str> = part_two_str.split("\n").collect();

    contents.pop();
    contents.push("[[2]]");
    contents.push("[[6]]");

    println!("Part 2 {}", part_two(contents));
}

fn parse_packet(packet: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut depth = 0;
    let mut split_index_start: Vec<usize> = Vec::new();
    let mut split_index_end: Vec<usize> = Vec::new();

    split_index_start.push(depth);

    let packet_chars: Vec<char> = packet.chars().collect();

    for i in 0..packet_chars.len() {
        match packet_chars[i] {
            '[' => depth = depth + 1,
            ']' => depth = depth - 1,
            ',' => {
                if depth == 0 {
                    split_index_end.push(i);
                    // Start the next part of the packet
                    split_index_start.push(i + 1);
                }
            }
            _ => (),
        }
    }

    split_index_end.push(packet.len());

    for s_i in 0..split_index_start.len() {
        if split_index_start[s_i] == split_index_end[s_i] {
            continue;
        }
        result.push(&packet[split_index_start[s_i]..split_index_end[s_i]]);
    }

    return result;
}

fn correct_order(a_str: &str, b_str: &str) -> Option<bool> {
    if a_str.len() == 1 && b_str.len() == 1 {
        if a_str.as_bytes()[0] > b_str.as_bytes()[0] {
            return Some(false);
        } else if a_str.as_bytes()[0] < b_str.as_bytes()[0] {
            return Some(true);
        }
        return None;
    }

    if a_str.len() > 1 && b_str.len() == 1 {
        return correct_order(a_str, format!("[{}]", b_str).as_str());
    }
    if a_str.len() == 1 && b_str.len() > 1 {
        return correct_order(format!("[{}]", a_str).as_str(), b_str);
    }

    let a_split = parse_packet(&a_str[1..a_str.len() - 1]);
    let b_split = parse_packet(&b_str[1..b_str.len() - 1]);

    for a_split_i in 0..a_split.len() {
        if b_split.len() == 0 || a_split_i > b_split.len() - 1 {
            return Some(false);
        }
        let result = correct_order(a_split[a_split_i], b_split[a_split_i]);
        if result.is_some() {
            return result;
        }
    }

    if a_split.len() < b_split.len() {
        return Some(true);
    }

    return None;
}

fn part_one(content: &Vec<&str>) -> usize {
    // each &str is a pair
    let valid: Vec<bool> = content
        .iter()
        .map(|v| {
            let packets: Vec<&str> = v.split("\n").collect();
            return correct_order(&packets[0], &packets[1]).unwrap();
        })
        .collect();

    let mut result = 0;

    for i in 0..valid.len() {
        if valid[i] == true {
            result = result + i + 1;
        }
    }

    return result;
}

fn part_two(mut content: Vec<&str>) -> usize {
    content.sort_by(|a, b| {
        let is_right_order = correct_order(a, b);
        if is_right_order.is_none() || is_right_order.unwrap() {
            return Ordering::Less;
        }
        return Ordering::Greater;
    });

    let mut two_index = 0;
    let mut six_index = 0;
    for i in 0..content.len() {
        if content[i] == "[[2]]" {
            two_index = i + 1;
        }
        if content[i] == "[[6]]" {
            six_index = i + 1;
        }
    }

    return two_index * six_index;
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let contents = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let contents: Vec<&str> = contents.split("\n\n").collect();

        assert_eq!(part_one(&contents), 13);
    }

    #[test]
    fn part_two_test() {
        let contents = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let contents = contents.replace("\n\n", "\n").replace("10", "a");

        let contents: Vec<&str> = contents.split("\n").collect();

        assert_eq!(part_two(contents), 140);
    }
}
