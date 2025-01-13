use regex::Regex;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;
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

fn get_valid_multiplications(memory: &str) -> Result<Vec<(i32, i32)>, ParseIntError> {
    let re = Regex::new(r"(?<mul>mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\))").unwrap();
    let matches: Result<Vec<(i32, i32)>, ParseIntError> = re
        .captures_iter(memory)
        .map(|x| {
            let a = x.name("a").unwrap().as_str().parse::<i32>().unwrap();
            let b = x.name("b").unwrap().as_str().parse::<i32>().unwrap();
            Ok((a, b))
        })
        .collect();

    matches
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
        let file_contents = read_file().unwrap();
        let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let valid_multiplications = get_valid_multiplications(memory).unwrap();
        let sum_of_multiplications = get_sum_of_multiplications(&valid_multiplications).unwrap();
        assert_eq!(sum_of_multiplications, 161);
    }

    #[test]
    fn input_case() {
        let memory = read_file().unwrap();
        let valid_multiplications = get_valid_multiplications(&memory).unwrap();
        let sum_of_multiplications = get_sum_of_multiplications(&valid_multiplications).unwrap();
        assert_eq!(sum_of_multiplications, 173785482);
    }
}
