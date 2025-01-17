fn main() {
    println!("Hello, world!");
}

fn get_frequency_of_xmas(input: &str) -> u32 {
    let word_search = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows: i32 = word_search.len().try_into().unwrap_or(0);
    let cols: i32 = word_search[0].len().try_into().unwrap_or(0);
    let dxdy: [(i32, i32); 8] = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    let mut frequency = 0;
    let word = "XMAS";

    for row in 0..rows {
        for col in 0..cols {
            for (dx, dy) in dxdy {
                let mut found = true;
                for i in 0..word.len() {
                    let rrow = row + (dx * (i as i32));
                    let ccol = col + (dy * (i as i32));
                    if rrow < 0 || rrow >= rows || ccol < 0 || ccol >= cols {
                        found = false;
                        break;
                    }
                    let word_value = word_search[rrow as usize][ccol as usize];
                    if word_value != word.chars().nth(i).unwrap() {
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
