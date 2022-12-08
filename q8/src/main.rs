use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let mut contents: Vec<&str> = contents.split("\n").collect();

    contents.pop();

    let trees: Vec<Vec<usize>> = parse_trees(&contents);

    println!("Part 1 {}", part_one(&trees));
    println!("Part 2 {}", part_two(&trees));
}

fn parse_trees(contents: &Vec<&str>) -> Vec<Vec<usize>> {
    return contents
        .iter()
        .map(|line| {
            return line
                .trim()
                .chars()
                .into_iter()
                .map(|v| {
                    return v.to_digit(10).unwrap() as usize;
                })
                .collect();
        })
        .collect();
}

fn part_one(tree_lines: &Vec<Vec<usize>>) -> usize {
    let max_column: usize = tree_lines.get(0).unwrap().len();
    let max_row: usize = tree_lines.len();

    let mut count = 0;

    let mut visited: Vec<Vec<bool>> = tree_lines
        .iter()
        .map(|trees| trees.iter().map(|_| false).collect())
        .collect();

    // Following trees are automatically visible [row][column]
    // [0][*] -> top
    // [MAX_ROW][*] -> bottom
    // [*][0] -> left
    // [*][MAX_COLUMN] -> right

    for i in 0..tree_lines.len() {
        let trees = tree_lines.get(i).unwrap();

        for j in 0..trees.len() {
            if !visited[i][j] {
                let tree: usize = trees[j];
                if is_tree_visible(&tree_lines, tree, i, j, max_row, max_column) {
                    count += 1;
                }
                visited[i][j] = true;
            }
        }
    }

    return count;
}

fn is_tree_visible(
    tree_lines: &Vec<Vec<usize>>,
    tree: usize,
    i: usize,
    j: usize,
    max_row: usize,
    max_column: usize,
) -> bool {
    return is_visible_from_top(&tree_lines, tree, i, j)
        || is_visible_from_left(&tree_lines, tree, i, j, max_column)
        || is_visible_from_bottom(&tree_lines, tree, i, j, max_row)
        || is_visible_from_right(&tree_lines, tree, i, j);
}

fn is_visible_from_top(tree_lines: &Vec<Vec<usize>>, tree: usize, i: usize, j: usize) -> bool {
    if i == 0 {
        return true;
    }
    // Go from the outside in and make sure all the values are < the current tree
    for val in 0..i {
        if tree <= tree_lines[val][j] {
            return false;
        }
    }

    return true;
}

fn is_visible_from_left(
    tree_lines: &Vec<Vec<usize>>,
    tree: usize,
    i: usize,
    j: usize,
    max_column: usize,
) -> bool {
    if j == max_column - 1 {
        return true;
    }

    // Go from the outside in and make sure all the values are < the current tree
    for val in j + 1..max_column {
        if tree <= tree_lines[i][val] {
            return false;
        }
    }

    return true;
}

fn is_visible_from_bottom(
    tree_lines: &Vec<Vec<usize>>,
    tree: usize,
    i: usize,
    j: usize,
    max_row: usize,
) -> bool {
    if i == max_row - 1 {
        return true;
    }
    // Go from the outside in and make sure all the values are < the current tree
    for val in i + 1..max_row {
        if tree <= tree_lines[val][j] {
            return false;
        }
    }

    return true;
}

fn is_visible_from_right(tree_lines: &Vec<Vec<usize>>, tree: usize, i: usize, j: usize) -> bool {
    if j == 0 {
        return true;
    }

    // Go from the outside in and make sure all the values are < the current tree
    for val in 0..j {
        if tree <= tree_lines[i][val] {
            return false;
        }
    }

    return true;
}

fn part_two(tree_lines: &Vec<Vec<usize>>) -> usize {
    let max_column: usize = tree_lines.get(0).unwrap().len();
    let max_row: usize = tree_lines.len();

    let mut max_scenic_score = 0;

    let mut visited: Vec<Vec<bool>> = tree_lines
        .iter()
        .map(|trees| trees.iter().map(|_| false).collect())
        .collect();

    // Following trees are automatically visible [row][column]
    // [0][*] -> top
    // [MAX_ROW][*] -> bottom
    // [*][0] -> left
    // [*][MAX_COLUMN] -> right

    for i in 0..tree_lines.len() {
        let trees = tree_lines.get(i).unwrap();

        for j in 0..trees.len() {
            if !visited[i][j] {
                let tree: usize = trees[j];
                let score: usize = get_scenic_score(&tree_lines, tree, i, j, max_row, max_column);
                if score > max_scenic_score {
                    max_scenic_score = score;
                }
                visited[i][j] = true;
            }
        }
    }

    return max_scenic_score;
}

fn get_scenic_score(
    tree_lines: &Vec<Vec<usize>>,
    tree: usize,
    i: usize,
    j: usize,
    max_row: usize,
    max_column: usize,
) -> usize {
    return scenic_score_top(&tree_lines, tree, i, j)
        * scenic_score_left(&tree_lines, tree, i, j, max_column)
        * scenic_score_bottom(&tree_lines, tree, i, j, max_row)
        * scenic_score_right(&tree_lines, tree, i, j);
}

fn scenic_score_top(tree_lines: &Vec<Vec<usize>>, tree: usize, i: usize, j: usize) -> usize {
    if i == 0 {
        return 0;
    }
    let mut score = 0;
    let mut val: i32 = (i - 1) as i32;

    while val >= 0 {
        score += 1;
        if tree <= tree_lines[val as usize][j] {
            break;
        }
        val -= 1;
    }

    return score;
}

fn scenic_score_left(
    tree_lines: &Vec<Vec<usize>>,
    tree: usize,
    i: usize,
    j: usize,
    max_column: usize,
) -> usize {
    if j == max_column - 1 {
        return 0;
    }

    let mut score = 0;
    let mut val: i32 = (j + 1) as i32;

    while val < max_column as i32 {
        score += 1;
        if tree <= tree_lines[i][val as usize] {
            break;
        }
        val += 1;
    }

    return score;
}

fn scenic_score_bottom(
    tree_lines: &Vec<Vec<usize>>,
    tree: usize,
    i: usize,
    j: usize,
    max_row: usize,
) -> usize {
    if i == max_row - 1 {
        return 0;
    }

    let mut score = 0;
    let mut val: i32 = (i + 1) as i32;

    while val < max_row as i32 {
        score += 1;
        if tree <= tree_lines[val as usize][j] {
            break;
        }
        val += 1;
    }

    return score;
}

fn scenic_score_right(tree_lines: &Vec<Vec<usize>>, tree: usize, i: usize, j: usize) -> usize {
    if j == 0 {
        return 0;
    }

    let mut score = 0;
    let mut val: i32 = (j - 1) as i32;

    while val >= 0 as i32 {
        score += 1;
        if tree <= tree_lines[i][val as usize] {
            break;
        }
        val -= 1;
    }

    return score;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{
        is_visible_from_bottom, is_visible_from_left, is_visible_from_right, is_visible_from_top,
        parse_trees, part_one, scenic_score_bottom, scenic_score_left, scenic_score_right,
        scenic_score_top,
    };

    #[test]
    fn part_one_test() {
        let contents =
            fs::read_to_string("./test.txt").expect("Should have been able to read the file");

        let mut contents: Vec<&str> = contents.split("\n").collect();

        contents.pop();

        let trees: Vec<Vec<usize>> = parse_trees(&contents);

        let result = part_one(&trees);
        assert_eq!(result, 21);
    }
    #[test]
    fn test_is_visible_from_top() {
        let trees: Vec<Vec<usize>> = vec![
            vec![8, 5, 4, 2, 1],
            vec![5, 5, 5, 5, 5],
            vec![9, 4, 2, 1, 3],
        ];

        assert_eq!(is_visible_from_top(&trees, trees[0][1], 0, 1), true);
        assert_eq!(is_visible_from_top(&trees, trees[1][1], 1, 1), false);
        assert_eq!(is_visible_from_top(&trees, trees[1][2], 1, 2), true);
        assert_eq!(is_visible_from_top(&trees, trees[2][0], 2, 0), true);
        assert_eq!(is_visible_from_top(&trees, trees[2][4], 2, 4), false);
    }
    #[test]
    fn test_is_visible_from_left() {
        let trees: Vec<Vec<usize>> = vec![vec![8, 3, 1, 2, 1]];

        assert_eq!(is_visible_from_left(&trees, trees[0][4], 0, 4, 5), true);
        assert_eq!(is_visible_from_left(&trees, trees[0][3], 0, 3, 5), true);
        assert_eq!(is_visible_from_left(&trees, trees[0][2], 0, 2, 5), false);
        assert_eq!(is_visible_from_left(&trees, trees[0][1], 0, 1, 5), true);
        assert_eq!(is_visible_from_left(&trees, trees[0][0], 0, 0, 5), true);
    }
    #[test]
    fn test_is_visible_from_bottom() {
        let trees: Vec<Vec<usize>> = vec![
            vec![8, 5, 4, 2, 1],
            vec![5, 5, 5, 5, 5],
            vec![5, 4, 2, 1, 3],
            vec![3, 4, 2, 1, 3],
        ];

        assert_eq!(is_visible_from_bottom(&trees, trees[0][0], 0, 0, 4), true);
        assert_eq!(is_visible_from_bottom(&trees, trees[1][0], 1, 0, 4), false);
        assert_eq!(is_visible_from_bottom(&trees, trees[2][0], 2, 0, 4), true);
        assert_eq!(is_visible_from_bottom(&trees, trees[3][0], 3, 0, 4), true);
    }
    #[test]
    fn test_is_visible_from_right() {
        let trees: Vec<Vec<usize>> = vec![vec![4, 5, 4, 2, 7]];

        assert_eq!(is_visible_from_right(&trees, trees[0][0], 0, 0), true);
        assert_eq!(is_visible_from_right(&trees, trees[0][1], 0, 1), true);
        assert_eq!(is_visible_from_right(&trees, trees[0][2], 0, 2), false);
        assert_eq!(is_visible_from_right(&trees, trees[0][3], 0, 3), false);
        assert_eq!(is_visible_from_right(&trees, trees[0][4], 0, 4), true);
    }

    #[test]
    fn test_scenic_score_top() {
        let trees: Vec<Vec<usize>> = vec![
            vec![2, 5, 4, 2, 1],
            vec![3, 5, 5, 5, 5],
            vec![2, 5, 5, 5, 5],
            vec![5, 5, 5, 5, 5],
        ];

        assert_eq!(scenic_score_top(&trees, trees[0][0], 0, 0), 0);
        assert_eq!(scenic_score_top(&trees, trees[1][0], 1, 0), 1);
        assert_eq!(scenic_score_top(&trees, trees[2][0], 2, 0), 1);
        assert_eq!(scenic_score_top(&trees, trees[3][0], 3, 0), 3);
    }
}
