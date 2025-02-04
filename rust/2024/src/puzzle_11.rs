use std::collections::HashMap;

struct StoneLine {
    stones: Vec<String>,
    cache: HashMap<(i64, isize), i64>,
}

impl StoneLine {
    fn new(input: &str) -> Self {
        let stones: Vec<String> = input.split_whitespace().map(String::from).collect();
        Self {
            stones,
            cache: HashMap::new(),
        }
    }
    fn simulate(&mut self, blinks: i32) -> i64 {
        let mut stones = 0;
        for i in 0..self.stones.len() {
            let v = &self.stones[i];
            stones += self.blink(v.parse::<i64>().unwrap(), blinks);
        }
        stones
    }

    fn blink(&mut self, number: i64, blinks: i32) -> i64 {
        if blinks == 0 {
            return 1;
        }

        if number == 0 {
            return self.blink(1, blinks - 1);
        }
        let nlen = number.ilog10() + 1;
        if nlen % 2 == 0 {
            if self.cache.contains_key(&(number, blinks as isize)) {
                return self.cache[&(number, blinks as isize)];
            }

            let left_half_num = number / 10_i64.pow(nlen / 2);
            let right_half_num = number % 10_i64.pow(nlen / 2);
            let s = self.blink(left_half_num, blinks - 1) + self.blink(right_half_num, blinks - 1);
            self.cache.insert((number, blinks as isize), s);
            return s;
        }

        let no_rule_num: i64 = number * 2024;
        return self.blink(no_rule_num, blinks - 1);
    }
}

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
        let mut stone_line = StoneLine::new(input);
        let output = stone_line.simulate(25);
        assert_eq!(output, 55312);
    }

    #[test]
    fn test_eleven_input_a() {
        let input = include_str!("../inputs/puzzle_11/input.txt");
        let mut stone_line = StoneLine::new(input);
        let output = stone_line.simulate(25);
        assert_eq!(output, 197357);
    }

    #[test]
    fn test_eleven_input_b() {
        let input = include_str!("../inputs/puzzle_11/input.txt");
        let mut stone_line = StoneLine::new(input);
        let output = stone_line.simulate(75);
        assert_eq!(output, 234568186890978);
    }

    #[test]
    fn test_log10() {
        let num: i64 = 1234567890;
        let len = num.ilog10();
        assert_eq!(len, 9);

        let num: i64 = 1234;
        let len = num.ilog10();
        assert_eq!(len, 3);
    }

    #[test]
    fn test_half_num() {
        let num: i64 = 1234;
        let nlen = num.ilog10() + 1;

        let left_half_num = num / 10_i64.pow(nlen / 2);
        assert_eq!(left_half_num, 12);

        let right_half_num = num % 10_i64.pow(nlen / 2);
        assert_eq!(right_half_num, 34);
    }

    // #[test]
    // fn test_ten_input_b() {
    //     let input = include_str!("../inputs/puzzle_10/input.txt");
    //     let output = ten(input, true);
    //     assert_eq!(output, 1686);
    // }
}
