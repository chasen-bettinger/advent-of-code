use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

struct Arcade {
    machines: Vec<Machine>,
}

impl Arcade {
    fn new(input: &str) -> Self {
        let button_re = Regex::new(
            r".*X\+(?<ax>[0-9]+), Y\+(?<ay>[0-9]+)
.*X\+(?<bx>[0-9]+), Y\+(?<by>[0-9]+)
Prize: X=(?<px>[0-9]+), Y=(?<py>[0-9]+)",
        )
        .unwrap();
        let mut machines: Vec<Machine> = vec![];
        let mut beg = 0;
        let mut e = 3;
        let split_input: Vec<&str> = input.split("\n").collect();
        while e < split_input.len() {
            let content = &split_input[beg..e].join("\n");
            // println!("{:#?}", content);
            let Some(values) = button_re.captures(content) else {
                panic!("bad")
            };
            let machine = Machine {
                button_a: Button {
                    x: values["ax"].parse::<i64>().unwrap(),
                    y: values["ay"].parse::<i64>().unwrap(),
                },
                button_b: Button {
                    x: values["bx"].parse::<i64>().unwrap(),
                    y: values["by"].parse::<i64>().unwrap(),
                },
                prize: Prize {
                    x: values["px"].parse::<i64>().unwrap(),
                    y: values["py"].parse::<i64>().unwrap(),
                },
            };
            machines.push(machine);
            beg += 4;
            e += 4;
        }

        Self { machines }
    }

    /*
       Visual Explanation

        Prize at (8400, 5400)
        Button A: (+94, +34) costs 3 tokens
        Button B: (+22, +67) costs 1 token

        The algorithm tries:
        a=0, b=0: (0×94 + 0×22, 0×34 + 0×67) = (0, 0) ❌
        a=0, b=1: (0×94 + 1×22, 0×34 + 1×67) = (22, 67) ❌
        a=0, b=2: (0×94 + 2×22, 0×34 + 2×67) = (44, 134) ❌
        ...
        a=80, b=40: (80×94 + 40×22, 80×34 + 40×67) = (8400, 5400) ✅
        Cost = 3×80 + 1×40 = 280 tokens
    */
    fn find_prize(&self, offset: i64) -> i64 {
        let mut total: i64 = 0;
        for machine in &self.machines {
            let x1 = machine.button_a.x;
            let x2 = machine.button_a.y;

            let y1 = machine.button_b.x;
            let y2 = machine.button_b.y;

            let z1 = machine.prize.x + offset;
            let z2 = machine.prize.y + offset;

            let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
            let a = (z1 - b * y1) / x1;
            if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
                continue;
            }
            total += a * 3 + b
        }
        total
    }
}
pub fn solution() -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

        let arcade = Arcade::new(input);
        let prize = arcade.find_prize(1);
        println!("prize :>> {}", prize)
    }

    #[test]
    fn test_basic_two() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

        let arcade = Arcade::new(input);
        let prize = arcade.find_prize(10000000000000);
        println!("{}", prize);
    }

    #[test]
    fn test_input_a() {
        let input = include_str!("../inputs/puzzle_13/input.txt");
        // println!("{}", input);
        let arcade = Arcade::new(input);
        let prize = arcade.find_prize(1);
        println!("{}", prize);
    }

    #[test]
    fn test_input_b() {
        let input = include_str!("../inputs/puzzle_13/input.txt");
        // println!("{}", input);
        let arcade = Arcade::new(input);
        let prize = arcade.find_prize(10000000000000);
        println!("{}", prize);
    }
}
