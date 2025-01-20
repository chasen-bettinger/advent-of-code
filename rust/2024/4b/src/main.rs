use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn find_xmas(input: &str) -> i32 {
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let r: i32 = lines.len().try_into().unwrap();
    let c: i32 = lines[0].len().try_into().unwrap();

    // 1. collect MAS positions, along with their 'A' value
    // 2. sum all the positions where 'A' values are equal
    // let mut positions = Vec::<(i32, i32)>::new();
    let mut positions = HashMap::<(i32, i32), i32>::new();

    for i in 0..r {
        for j in 0..c {
            match lines[i as usize][j as usize] {
                'M' => {
                    [(1, 1), (1, -1), (-1, 1), (-1, -1)]
                        .iter()
                        .for_each(|(dx, dy)| {
                            let mut a_index = (0, 0);
                            let valid_word = (1..3).all(|k| {
                                let rr = i + dx * k;
                                let cc = j + dy * k;
                                if rr < 0 || rr >= r || cc < 0 || cc >= c {
                                    return false;
                                }
                                let value = lines[rr as usize][cc as usize];
                                let current_index: usize = k.try_into().unwrap();
                                let letter = "MAS".chars().nth(current_index).unwrap();
                                if letter == 'A' {
                                    a_index = (rr, cc);
                                }
                                value == letter
                            });
                            if !valid_word {
                                return;
                            }
                            if positions.contains_key(&a_index) {
                                *positions.entry(a_index).or_insert(0) += 1;
                            } else {
                                positions.insert(a_index, 1);
                            }
                        });
                }
                _ => {}
            }
        }
    }

    positions
        .iter()
        .filter(|(_, v)| **v == 2)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let result = find_xmas(input);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let result = find_xmas(input);
        assert_eq!(result, 1936);
    }
}
