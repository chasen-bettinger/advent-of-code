fn clear_leading_zeros(s: &str) -> String {
    let mut s = s.to_string();
    while s.starts_with('0') && s.len() > 1 {
        s.remove(0);
    }
    s
}

fn eleven(input: &str, blinks: i32) -> i64 {
    let split_str: Vec<String> = input.split_whitespace().map(String::from).collect();
    let mut stones = split_str;
    for _ in 0..blinks {
        let mut next_stones = vec![];
        for i in 0..stones.len() {
            let v = &stones[i];
            let num = v.parse::<i64>().unwrap();
            if num == 0 {
                next_stones.push(String::from("1"));
                continue;
            }
            if v.len() % 2 == 0 {
                let half_num = v.len() / 2;
                let left_half_str = &v[0..half_num];
                let right_half_str = &v[half_num..v.len()];
                next_stones.push(clear_leading_zeros(left_half_str));
                next_stones.push(clear_leading_zeros(right_half_str));
                continue;
            }

            let no_rule_num: i64 = num * 2024;
            next_stones.push(no_rule_num.to_string());
        }
        stones = next_stones;
    }

    stones.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eleven() {
        let input = "125 17";
        let output = eleven(input, 25);
        assert_eq!(output, 55312);
    }

    #[test]
    fn test_ten_input_a() {
        let input = include_str!("../inputs/puzzle_11/input.txt");
        let output = eleven(input, 25);
        assert_eq!(output, 197357);
    }

    // #[test]
    // fn test_ten_input_b() {
    //     let input = include_str!("../inputs/puzzle_10/input.txt");
    //     let output = ten(input, true);
    //     assert_eq!(output, 1686);
    // }
}
