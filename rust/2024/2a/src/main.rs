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

fn get_safe_reports(reports: &Vec<Vec<i32>>) -> Result<i32, Box<dyn Error>> {
    let mut safe_reports = 0;

    for i in 0..reports.len() {
        let report = reports.get(i).unwrap();
        let mut is_safe = true;
        let mut is_increasing = true;
        if report.len() == 0 {
            continue;
        }
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
        if is_safe {
            // println!("safe report: {:?}", report);
            safe_reports += 1;
        }
    }

    println!("{:?}", safe_reports);
    Ok(safe_reports)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(get_safe_reports(&reports).unwrap(), 2);
    }

    #[test]
    fn input_case() {
        let input = parse_input();
        assert_eq!(get_safe_reports(&input).unwrap(), 359);
    }
}
