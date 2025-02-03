use std::collections::HashMap;

#[derive(Clone)]
enum Direction {
    All,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct DirectionLedger {
    direction: Direction,
    coordinate: (isize, isize),
}

struct StackValue {
    prev_coordinate: (usize, usize),
    prev_direction: Direction,
}

const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (0, 1);
const ALL_DIRECTIONS: [DirectionLedger; 4] = [
    DirectionLedger {
        direction: Direction::Up,
        coordinate: UP,
    },
    DirectionLedger {
        direction: Direction::Down,
        coordinate: DOWN,
    },
    DirectionLedger {
        direction: Direction::Left,
        coordinate: LEFT,
    },
    DirectionLedger {
        direction: Direction::Right,
        coordinate: RIGHT,
    },
];
const UP_DIRECTIONS: [DirectionLedger; 3] = [
    DirectionLedger {
        direction: Direction::Up,
        coordinate: UP,
    },
    DirectionLedger {
        direction: Direction::Left,
        coordinate: LEFT,
    },
    DirectionLedger {
        direction: Direction::Right,
        coordinate: RIGHT,
    },
];
const DOWN_DIRECTIONS: [DirectionLedger; 3] = [
    DirectionLedger {
        direction: Direction::Down,
        coordinate: DOWN,
    },
    DirectionLedger {
        direction: Direction::Left,
        coordinate: LEFT,
    },
    DirectionLedger {
        direction: Direction::Right,
        coordinate: RIGHT,
    },
];
const LEFT_DIRECTIONS: [DirectionLedger; 3] = [
    DirectionLedger {
        direction: Direction::Up,
        coordinate: UP,
    },
    DirectionLedger {
        direction: Direction::Down,
        coordinate: DOWN,
    },
    DirectionLedger {
        direction: Direction::Left,
        coordinate: LEFT,
    },
];
const RIGHT_DIRECTIONS: [DirectionLedger; 3] = [
    DirectionLedger {
        direction: Direction::Up,
        coordinate: UP,
    },
    DirectionLedger {
        direction: Direction::Down,
        coordinate: DOWN,
    },
    DirectionLedger {
        direction: Direction::Right,
        coordinate: RIGHT,
    },
];

fn get_direction_ledger(dir: Direction) -> Vec<DirectionLedger> {
    match dir {
        Direction::All => ALL_DIRECTIONS.to_vec(),
        Direction::Up => UP_DIRECTIONS.to_vec(),
        Direction::Down => DOWN_DIRECTIONS.to_vec(),
        Direction::Left => LEFT_DIRECTIONS.to_vec(),
        Direction::Right => RIGHT_DIRECTIONS.to_vec(),
    }
}

fn ten(input: &str, distinct: bool) -> i64 {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut score = 0;
    let mut trailhead_ledger: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for r in 0..lines.len() {
        for c in 0..lines[r].len() {
            let v = lines[r][c];
            match v {
                0 => {
                    let mut stack: Vec<StackValue> = vec![];
                    stack.push(StackValue {
                        prev_coordinate: (r, c),
                        prev_direction: Direction::All,
                    });

                    while !stack.is_empty() {
                        let StackValue {
                            prev_coordinate: (prev_x, prev_y),
                            prev_direction,
                        } = stack.pop().unwrap();
                        let v = lines[prev_x][prev_y];
                        let direction_ledger = get_direction_ledger(prev_direction);
                        for direction_coord in direction_ledger {
                            let x = prev_x as i32 + direction_coord.coordinate.0 as i32;
                            let y = prev_y as i32 + direction_coord.coordinate.1 as i32;
                            if x < 0
                                || y < 0
                                || x >= lines.len() as i32
                                || y >= lines[0].len() as i32
                            {
                                continue;
                            }

                            let step_value = lines[x as usize][y as usize];

                            if step_value == 9 && v == 8 {
                                if !distinct {
                                    let trailhead_ledger_value = trailhead_ledger
                                        .entry((r as usize, c as usize))
                                        .or_insert(vec![]);

                                    if trailhead_ledger_value.contains(&(x as usize, y as usize)) {
                                        continue;
                                    }

                                    trailhead_ledger_value.push((x as usize, y as usize));
                                }

                                score += 1;
                                continue;
                            }

                            if step_value != v + 1 {
                                continue;
                            }

                            stack.push(StackValue {
                                prev_coordinate: (x as usize, y as usize),
                                prev_direction: direction_coord.direction,
                            });
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ten() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let output = ten(input, false);
        assert_eq!(output, 36);
    }

    #[test]
    fn test_ten_input_a() {
        let input = include_str!("../inputs/puzzle_10/input.txt");
        let output = ten(input, false);
        assert_eq!(output, 717);
    }

    #[test]
    fn test_ten_input_b() {
        let input = include_str!("../inputs/puzzle_10/input.txt");
        let output = ten(input, true);
        assert_eq!(output, 1686);
    }
}
