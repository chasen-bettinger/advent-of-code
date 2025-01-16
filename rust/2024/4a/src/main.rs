use regex::Regex;
use std::error::Error;
use std::fs;
fn read_file() -> std::io::Result<String> {
    fs::read_to_string("input.txt")
}

//fn read_file() -> std::io::Result<()> {
//    let file = std::fs::File::open("input.txt")?;
//    let reader = std::io::BufReader::new(file);
//    for line in reader.lines() {
//        println!("{:?}", line);
//    }
//    Ok(())
//}

fn parse_input() -> Vec<Vec<i32>> {
    let output = read_file().unwrap();
    let mut reports: Vec<Vec<i32>> = Vec::new();
    output.split("\n").for_each(|line| {
        let mut report: Vec<i32> = Vec::new();
        line.split(" ").for_each(|num| {
            let Ok(y): Result<i32, _> = num.parse() else {
                return;
            };
            report.push(y);
        });

        reports.push(report);
    });

    reports
}

/*
So, a report only counts as safe if both of the following are true:

    The levels are either all increasing or all decreasing.
    Any two adjacent levels differ by at least one and at most three.

In the example above, the reports can be found safe or unsafe by checking those rules:

*/

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut is_safe = true;
    let mut is_increasing = true;
    for j in 0..report.len() {
        if j == 0 {
            continue;
        }
        let current_value = report.get(j).unwrap();
        let previous_value = report.get(j - 1).unwrap();
        let difference = current_value - previous_value;
        let sign = difference.signum();
        if j == 1 {
            if sign == 0 {
                is_safe = false;
                break;
            }

            if sign == -1 {
                is_increasing = false;
            }
        }

        if j > 1 {
            if sign == 0 {
                is_safe = false;
                break;
            }

            let increasing = sign == 1;
            if is_increasing != increasing {
                is_safe = false;
                break;
            }
        }

        let value = difference.abs();

        if value < 1 || value > 3 {
            is_safe = false;
            break;
        }
    }
    is_safe
}

fn tolerate_report_failure(report: &Vec<i32>) -> bool {
    let original_report = report;

    if is_report_safe(original_report) {
        return true;
    }

    for i in 0..report.len() {
        let mut r = original_report.clone();
        r.remove(i);
        if is_report_safe(&r) {
            return true;
        }
    }
    false
}

fn traverse_word_search(
    word_search: &Vec<Vec<char>>,
    starting_row_index: usize,
    starting_column_index: usize,
    direction: &String,
) -> Result<bool, Box<dyn Error>> {
    let get_value = |row_index: usize, column_index: usize| {
        let Some(row) = word_search.get(row_index) else {
            return None;
        };
        let Some(cell) = row.get(column_index) else {
            return None;
        };
        Some(cell)
    };
    if direction == "right" {
        let second_value = get_value(starting_row_index, starting_column_index + 1);
        if second_value != Some(&'M') {
            return Err("Second value is not M".into());
        }

        let third_value = get_value(starting_row_index, starting_column_index + 2);
        if third_value != Some(&'A') {
            return Err("Third value is not A".into());
        }
        let fourth_value = get_value(starting_row_index, starting_column_index + 3);
        if fourth_value != Some(&'S') {
            return Err("Fourth value is not S".into());
        }

        return Ok(true);
    }

    Ok(false)
}

fn parse_word_search(word_search: &str) -> Result<i32, Box<dyn Error>> {
    // let word_search: Vec<Vec<char>> = Vec::new();
    let mut matches = 0;
    let mut character_groups: Vec<Vec<char>> = Vec::new();
    let mut character_group: Vec<char> = Vec::new();
    word_search.chars().for_each(|line| {
        if line == '\n' && character_group.len() == 0 {
            return;
        }
        if line == '\n' {
            character_groups.push(character_group.clone());
            character_group = Vec::new();
        } else {
            character_group.push(line);
        }
    });

    for (row_index, character_group) in character_groups.iter().enumerate() {
        for (column_index, character) in character_group.iter().enumerate() {
            if character == &'X' {
                let found_value = traverse_word_search(
                    &character_groups,
                    row_index,
                    column_index,
                    &String::from("right"),
                );

                if found_value.is_ok() {
                    matches += 1;
                }
            }
        }
    }

    // println!("{:?}", character_groups);
    Ok(matches)
}

fn get_sum_of_multiplications(
    valid_multiplications: &Vec<(i32, i32)>,
) -> Result<i32, Box<dyn Error>> {
    let mut sum_of_multiplications = 0;
    valid_multiplications.iter().for_each(|(a, b)| {
        sum_of_multiplications += a * b;
    });
    Ok(sum_of_multiplications)
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
        let word_search = parse_word_search(word_search).unwrap();
        // let sum_of_multiplications = get_sum_of_multiplications(&valid_multiplications).unwrap();
        assert_eq!(word_search, 18);
    }

    // #[test]
    // fn input_case() {
    //     let memory = read_file().unwrap();
    //     let valid_multiplications = get_valid_multiplications(&memory).unwrap();
    //     let sum_of_multiplications = get_sum_of_multiplications(&valid_multiplications).unwrap();
    //     assert_eq!(sum_of_multiplications, 83158140);
    // }
}
