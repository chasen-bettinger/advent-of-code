use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn get_coords(input: &str) -> Vec<(i32, i32)> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut coords: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut insert = |v: &char, r: &usize, c: &usize| {
        if coords.contains_key(v) {
            coords.get_mut(v).unwrap().push((*r, *c));
        } else {
            coords.insert(*v, vec![(*r, *c)]);
        }
    };

    for r in 0..lines.len() {
        for c in 0..lines[r].len() {
            let v = lines[r][c];
            match v {
                '.' => {}
                v => {
                    insert(&v, &r, &c);
                }
                _ => {}
            }
        }
    }

    let mut pairs = vec![];
    let mut add_pair = |r: &i32, c: &i32| {
        if *r < 0 || *c < 0 || *r >= lines.len() as i32 || *c >= lines[*r as usize].len() as i32 {
            return;
        }
        if pairs.contains(&(*r, *c)) {
            return;
        }
        pairs.push((*r, *c));
    };

    for (k, v) in &coords {
        for pair_one in v {
            let co = coords.clone();
            let s = co.get(&k).unwrap();
            for pair_two in s {
                if pair_one.0 == pair_two.0 && pair_one.1 == pair_two.1 {
                    continue;
                }
                let (r1, c1) = pair_one;
                let (r2, c2) = pair_two;
                let r1 = *r1 as i32;
                let c1 = *c1 as i32;
                let r2 = *r2 as i32;
                let c2 = *c2 as i32;
                //
                let dr = (r2 - r1) as i32;
                let dc = (c2 - c1) as i32;
                // let dr = dr.abs();
                // let dc = dc.abs();

                let is_left = c1 < c2 && r1 < r2;
                if is_left {
                    let north_nr = r1 - dr;
                    let north_nc = c1 - dc;
                    add_pair(&north_nr, &north_nc);

                    let south_nr = r2 + dr;
                    let south_nc = c2 + dc;
                    add_pair(&south_nr, &south_nc);
                } else {
                    let north_nr = r1 - dr;
                    let north_nc = c1 - dc;
                    add_pair(&north_nr, &north_nc);

                    let south_nr = r2 + dr;
                    let south_nc = c2 + dc;
                    add_pair(&south_nr, &south_nc);
                }
            }
        }
    }

    // dbg!(&pairs);
    // dbg!(&pairs.len());

    pairs
}

fn make_grid(coords: &Vec<(i32, i32)>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..10 {
        let mut row: Vec<char> = Vec::new();
        for j in 0..10 {
            if coords.contains(&(i, j)) {
                print!("#");
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
    fn test_get_coords() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        let coords = get_coords(input);
        let grid = make_grid(&coords);
        // dbg!(&grid);
        assert_eq!(coords.len(), 14);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let coords = get_coords(input);
        // let grid = make_grid(&coords);
        // dbg!(&grid);
        assert_eq!(coords.len(), 400);
    }
}
