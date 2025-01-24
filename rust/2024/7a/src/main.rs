fn main() {
    println!("Hello, world!");
}

fn matches_target(numbers: &Vec<i64>, target: i64, operations: &Vec<Vec<&str>>) -> bool {
    for ops in operations {
        let mut current = numbers[0];
        (1..numbers.len()).for_each(|i| {
            if ops[i - 1] == "+" {
                current += numbers[i];
            } else {
                current *= numbers[i];
            }
        });
        if current == target {
            return true;
        }
    }

    false
}

fn get_calibrations(numbers: &Vec<(Vec<i64>, i64)>) -> i64 {
    numbers
        .iter()
        .filter(|(numbers, target)| {
            let operations = numbers.len() - 1;
            let permutations = 2_i64.pow(operations as u32);
            let mut all_ops = vec![];

            for i in 0..permutations {
                let mut ops = vec![];
                for j in 0..operations {
                    ops.push(if i & (1 << j) != 0 { "+" } else { "*" });
                }
                all_ops.push(ops);
            }
            matches_target(numbers, *target, &all_ops)
        })
        .map(|(_, target)| target)
        .sum()
}

fn get_numbers(input: &str) -> Vec<(Vec<i64>, i64)> {
    input
        .lines()
        .map(|line| {
            let (target, numbers) = line.split_once(": ").unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let target = target.parse().unwrap();
            (numbers, target)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let numbers = get_numbers(input);
        let result = get_calibrations(&numbers);
        assert_eq!(result, 3749);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        let numbers = get_numbers(input);
        let result = get_calibrations(&numbers);
        assert_eq!(result, 7579994664753);
    }
}
