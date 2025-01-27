fn main() {
    println!("Hello, world!");
}

fn matches_target(numbers: &Vec<i64>, target: i64, operations: &Vec<Vec<String>>) -> bool {
    for ops in operations {
        let mut current = numbers[0];
        (1..numbers.len()).for_each(|i| match ops[i - 1].as_str() {
            "+" => current += numbers[i],
            "*" => current *= numbers[i],
            "||" => {
                let c = current.to_string();
                let n = numbers[i].to_string();
                let r = format!("{}{}", c, n);
                let v = r.parse::<i64>().unwrap();

                current = v;
            }
            _ => unreachable!(),
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
            // let permutations = 3_i64.pow(operations as u32);
            // let mut all_ops = vec![];

            let mut all_ops = vec![];
            let ops = ["+".to_string(), "*".to_string(), "||".to_string()];

            // Generate all combinations with repetition
            // for i in 0..ops.len() {
            //     for j in 0..ops.len() {
            //         all_ops.push(vec![ops[i], ops[j]]);
            //     }
            // }
            fn generate_combinations(
                current: Vec<String>,
                depth: usize,
                ops: &[String],
                all_ops: &mut Vec<Vec<String>>,
            ) {
                if current.len() == depth {
                    all_ops.push(current);
                    return;
                }
                for op in ops {
                    let mut new_current = current.clone();
                    new_current.push(op.clone());
                    generate_combinations(new_current, depth, ops, all_ops);
                }
            }
            generate_combinations(vec![], operations, &ops, &mut all_ops);

            // dbg!(&all_ops);
            // let k = ["+", "*", "||"]
            //     .iter()
            //     .permutations(operations + 1)
            //     .collect_vec();
            // dbg!(&k);

            // permutations: +, *, ||
            // for i in 0..permutations {
            //     let mut ops = vec![];
            //     for j in 0..operations {
            //         // dbg!(i, j);
            //         // i: 6 (0110)
            //         // j: 1 (0001)
            //         // &: bitwise AND:
            //         //  - compares each bit in i and j
            //         // .- returns non-zero only if the jth bit in i is 1
            //         // <<: bitwise left shift
            //         //  - takes number 1 (0001)
            //         // .- shifts it left j positions (0010)

            //         // 0110 & 0010 = 0010
            //         // 0010 & 0001 = 0000
            //         // 1000 & 1000 = 1000

            //         // let v = i & (1 << j);
            //         // dbg!(v);
            //         // println!("=====");
            //         ops.push(if i & (1 << j) != 0 { "+" } else { "*" });
            //     }
            //     all_ops.push(ops);
            // }
            // dbg!(&all_ops);
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
        assert_eq!(result, 11387);
    }

    // #[ignore]
    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        let numbers = get_numbers(input);
        let result = get_calibrations(&numbers);
        assert_eq!(result, 438027111276610);
    }

    #[test]
    fn bitwise_and() {
        // 0111 = 7
        // 1000 = 8
        let k = 0b0111 & 0b0100;
        assert_eq!(k, 0b0100);
    }
}
