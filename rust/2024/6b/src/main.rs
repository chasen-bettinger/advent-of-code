use itertools::Itertools;
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

fn is_out_of_bounds(r: i32, c: i32, grid: &Vec<Vec<char>>) -> bool {
    return r < 0 || r >= grid.len() as i32 || c < 0 || c >= grid[0].len() as i32;
}

fn get_distinct_positions(input: &str) -> Vec<(i32, i32)> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let r: i32 = grid.len() as i32;
    let c: i32 = grid[0].len() as i32;
    let (mut start_r, mut start_c) = get_starting_position(&grid);
    let directions: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut hash_trail: Vec<(i32, i32)> = Vec::new();
    let mut current_direction = 0;
    let mut current_r = start_r;
    let mut current_c = start_c;
    let mut potential_barriers: Vec<(i32, i32)> = Vec::new();
    // let mut visited: Vec<(usize, usize)> = Vec::from([(current_r, current_c)]);
    loop {
        let direction = directions[current_direction];
        let new_r = direction.0 + current_r as i32;
        let new_c = direction.1 + current_c as i32;

        // we're out of bounds, end of path
        if is_out_of_bounds(new_r, new_c, &grid) {
            break;
        }

        let new_position = grid[new_r as usize][new_c as usize];
        if new_position == '#' {
            current_direction = (current_direction + 1) % 4;
            hash_trail.push((new_r as i32, new_c as i32));
            // if hash_trail.len() > 3 {
            //     hash_trail.remove(0);
            // }
            continue;
        }

        current_r = new_r as usize;
        current_c = new_c as usize;

        let considered_turn = (current_direction + 1) % 4;
        let considered_turn_r = directions[considered_turn].0 + current_r as i32;
        let considered_turn_c = directions[considered_turn].1 + current_c as i32;
        if is_out_of_bounds(considered_turn_r, considered_turn_c, &grid) {
            continue;
        }
        if hash_trail.len() < 3 {
            continue;
        }

        for trail in &hash_trail {
            let hash_trail_r = trail.0;
            let hash_trail_c = trail.1;

            let current_r_distance = current_r as i32 - hash_trail_r;
            let current_c_distance = current_c as i32 - hash_trail_c;

            let considered_turn_r_distance = considered_turn_r as i32 - hash_trail_r;
            let considered_turn_c_distance = considered_turn_c as i32 - hash_trail_c;

            let closer_r = considered_turn_r_distance <= current_r_distance;
            let closer_c = considered_turn_c_distance <= current_c_distance;
            let same_r = considered_turn_r == hash_trail_r;
            let same_c = considered_turn_c == hash_trail_c;

            if current_r == 8 && current_c == 4 {
                dbg!("hello_world");
            }

            if closer_r && closer_c && (same_r || same_c) {
                potential_barriers.push((
                    direction.0 + current_r as i32,
                    direction.1 + current_c as i32,
                ));
            }
        }
    }

    dbg!(&potential_barriers);
    // potential_barriers.len() as i32
    potential_barriers
}

fn walk(
    m: &[Vec<u8>],
    mut r: usize,
    mut c: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; m[0].len()]; m.len()];
    let mut d = 0;
    loop {
        if seen[r][c][d] {
            return None;
        }
        seen[r][c][d] = true;
        let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][d];
        let (rr, cc) = (r + dr as usize, c + dc as usize);
        if !(0..m.len()).contains(&rr) || !(0..m[0].len()).contains(&cc) {
            if !return_squares {
                return Some(Vec::new());
            }
            let visited = (0..m.len())
                .cartesian_product(0..m[0].len())
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();
            return Some(visited);
        }
        if m[rr][cc] == b'#' {
            d = (d + 1) % 4;
        } else {
            (r, c) = (rr, cc);
        }
    }
}

/*
m -> matrix
r -> starting row
c -> starting column
return_squares -> return all of the visited cells
*/
fn chasen_walk(
    m: &[Vec<u8>],
    mut r: usize,
    mut c: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; m[0].len()]; m.len()];
    let mut d = 0;
    loop {
        if seen[r][c][d] {
            return None;
        }
        seen[r][c][d] = true;
        let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][d];
        let (rr, cc) = (r + dr as usize, c + dc as usize);
        let oob_r = !(0..m.len()).contains(&rr);
        let oob_c = !(0..m[0].len()).contains(&cc);
        if oob_r || oob_c {
            // the visited squares are unimportant
            if !return_squares {
                return Some(Vec::new());
            }

            // any node visited at any direction
            let visited = (0..m.len())
                .cartesian_product(0..m[0].len())
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();

            return Some(visited);
        }
        if m[rr][cc] == b'#' {
            d = (d + 1) % 4;
        } else {
            (r, c) = (rr, cc);
        }
    }
}

fn make_grid(coords: &Vec<(i32, i32)>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..10 {
        let mut row: Vec<char> = Vec::new();
        for j in 0..10 {
            if coords.contains(&(i, j)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!("");
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        //         let input = r#"....#.....
        // .........#
        // ..........
        // ..#.......
        // .......#..
        // ..........
        // .#..^.....
        // ........#.
        // #.........
        // ......#..."#;

        let input = include_str!("../input.txt");

        let mut m = input
            .lines()
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<_>>();
        let (sr, sc) = (0..m.len())
            .cartesian_product(0..m[0].len())
            .find(|&(r, c)| m[r][c] == b'^')
            .unwrap();
        // p1 is a list of coordinates that were traversed during the iteration
        let p1 = walk(&m, sr, sc, true).unwrap();

        let p2 = p1
            .iter()
            .filter(|&&(r, c)| {
                m[r][c] = b'#';
                // if walk returns none, it means that the cursor
                // has traversed over a cell in a direction it has
                // already passed before
                let ok = walk(&m, sr, sc, false).is_none();
                m[r][c] = b'.';
                ok
            })
            .count();
        // let result = walk(&m, 8, 4, false).unwrap();
        // dbg!(&result);
        // let result = get_distinct_positions(input);
        // let grid = make_grid(&result);
        // dbg!(&grid);
        // assert!(false);
        // dbg!(&grid);
        assert_eq!(p2, 6);
    }

    // #[test]
    // fn test_input() {
    //     let input = include_str!("../input.txt");
    //     let result = get_distinct_positions(input);
    //     assert_eq!(result, 4696);
    // }
}
