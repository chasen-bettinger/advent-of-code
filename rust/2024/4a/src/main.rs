use std::error::Error;
use std::fs;
fn read_file() -> std::io::Result<String> {
    fs::read_to_string("input.txt")
}

fn parse_word_search(word_search: &str) -> Result<i32, Box<dyn Error>> {
    // TODO: what does lines do?
    let character_groups: Vec<Vec<char>> = word_search
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    // this pattern fails because it misses the last new line.
    // word_search.chars().for_each(|line| {
    //     if line == '\n' && character_group.len() == 0 {
    //         return;
    //     }
    //     if line == '\n' {
    //         character_groups.push(character_group.clone());
    //         character_group = Vec::new();
    //     } else {
    //         character_group.push(line);
    //     }
    // });

    let rows: i32 = character_groups.len().try_into().unwrap_or(0);
    let columns: i32 = character_groups[0].len().try_into().unwrap_or(0);
    let target = "XMAS".chars().collect::<Vec<char>>();

    let dxdy: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut matches = 0;
    for i in 0..rows {
        for j in 0..columns {
            for (dx, dy) in dxdy {
                let mut found = true;
                for k in 0..target.len() {
                    // k is the 'step' of the search
                    // it moves the pointer in the direction of dx, dy
                    // without it, we wouldn't be progressing in the
                    // direction of the search.
                    let x = i + dx * k as i32;
                    let y = j + dy * k as i32;

                    if x < 0
                        || y < 0
                        || x >= rows
                        || y >= columns
                        || character_groups[x as usize][y as usize] != target[k]
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    matches += 1;
                }
            }
        }
    }
    Ok(matches)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let word_search = r#"
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
        let xmas_findings = parse_word_search(word_search).unwrap();
        assert_eq!(xmas_findings, 18);
    }

    #[test]
    fn input_case() {
        let word_search = read_file().unwrap();
        let xmas_findings = parse_word_search(&word_search).unwrap();
        assert_eq!(xmas_findings, 2633);
    }
}
