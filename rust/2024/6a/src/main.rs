fn main() {
    println!("Hello, world!");
}

fn get_starting_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '^' {
                return (row, col);
            }
        }
    }
    (0, 0)
}

fn get_distinct_positions(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let r: i32 = grid.len() as i32;
    let c: i32 = grid[0].len() as i32;
    let (mut start_r, mut start_c) = get_starting_position(&grid);
    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_direction = 0;
    let mut current_r = start_r;
    let mut current_c = start_c;
    let mut visited: Vec<(usize, usize)> = Vec::from([(current_r, current_c)]);
    loop {
        let direction = directions[current_direction];
        let new_r = direction.0 + current_r as i32;
        let new_c = direction.1 + current_c as i32;
        // we're out of bounds, end of path
        if new_r < 0 || new_r >= r || new_c < 0 || new_c >= c {
            break;
        }

        let new_position = grid[new_r as usize][new_c as usize];
        if new_position == '#' {
            current_direction = (current_direction + 1) % 4;
            continue;
        }

        current_r = new_r as usize;
        current_c = new_c as usize;
        let new_coordinates = (current_r, current_c);
        if !visited.contains(&new_coordinates) {
            visited.push(new_coordinates);
        }
    }

    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let result = get_distinct_positions(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let result = get_distinct_positions(input);
        assert_eq!(result, 4696);
    }
}
