fn main() {
    println!("Hello, world!");
}

fn get_frequency_of_xmas(input: &str) -> i32 {
    let search = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows: i32 = search.len() as i32;
    let cols: i32 = search[0].len() as i32;
    let mut frequency = 0;
    let WORD = "XMAS";
    let dxdy = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    for row in 0..rows {
        for col in 0..cols {
            for (dx, dy) in dxdy {
                let mut found = true;
                for (i, c) in WORD.chars().enumerate() {
                    let x = row + (dx * i as i32);
                    let y = col + (dy * i as i32);
                    let overflow_x = x < 0 || x >= rows;
                    let overflow_y = y < 0 || y >= cols;
                    if overflow_x || overflow_y {
                        found = false;
                        break;
                    }
                    let curr_value = search[x as usize][y as usize];
                    if curr_value != c {
                        found = false;
                        break;
                    }
                }
                if found {
                    frequency += 1;
                }
            }
        }
    }
    frequency
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_simple() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let frequency = get_frequency_of_xmas(input);
        assert_eq!(frequency, 18);
    }
    #[test]
    fn test_input() {
        let input = fs::read_to_string("input.txt").unwrap();
        let frequency = get_frequency_of_xmas(&input);
        assert_eq!(frequency, 2633);
    }
}
