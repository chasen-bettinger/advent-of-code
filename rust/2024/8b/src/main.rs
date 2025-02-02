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
    let mut add_pair = |r: &i32, c: &i32| -> bool {
        if *r < 0 || *c < 0 || *r >= lines.len() as i32 || *c >= lines[*r as usize].len() as i32 {
            return false;
        }
        if pairs.contains(&(*r, *c)) {
            return true;
        }
        pairs.push((*r, *c));
        true
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

                let mut nr = r1;
                let mut nc = c1;

                loop {
                    let mr = nr - dr;
                    let mc = nc - dc;
                    let ok = add_pair(&mr, &mc);
                    if !ok {
                        break;
                    }
                    nr = mr;
                    nc = mc;
                }

                let mut nr = r2;
                let mut nc = c2;
                loop {
                    let mr = nr + dr;
                    let mc = nc + dc;
                    let ok = add_pair(&mr, &mc);
                    if !ok {
                        break;
                    }
                    nr = mr;
                    nc = mc;
                }

                add_pair(&r1, &c1);
                add_pair(&r2, &c2);
                // if added_n >= 2 || added_s >= 2 {
                // }
            }
        }
    }

    // dbg!(&pairs);
    // dbg!(&pairs.len());

    pairs
}

fn make_grid(coords: &Vec<(i32, i32)>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..12 {
        let mut row: Vec<char> = Vec::new();
        for j in 0..12 {
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
        assert_eq!(coords.len(), 34);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let coords = get_coords(input);
        // let grid = make_grid(&coords);
        // dbg!(&grid);
        assert_eq!(coords.len(), 1);
    }
}
