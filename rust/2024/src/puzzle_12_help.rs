use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Region {
    plant: String,
    area: usize,
    horizonzal_sides: Vec<(i32, i32)>,
    vertical_sides: Vec<(i32, i32)>,
    hsl: usize,
    vsl: usize,
    sides: usize,
}

fn fill_region(
    sx: i32,
    sy: i32,
    grid: &[u8],
    width: usize,
    height: usize,
    seen: &mut [bool],
) -> Region {
    let mut area = 0;
    let mut horizonzal_sides = Vec::new();
    let mut vertical_sides = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((sx, sy));
    seen[sy as usize * width + sx as usize] = true;
    let plant = grid[sy as usize * width + sx as usize];
    let plant = &[plant];
    let plant = std::str::from_utf8(plant).unwrap();

    while let Some((x, y)) = queue.pop_front() {
        let c = grid[y as usize * width + x as usize];
        area += 1;

        for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x + d.0;
            let ny: i32 = y + d.1;
            let ni = ny as i32 * width as i32 + nx as i32;
            if nx >= 0
                && ny >= 0
                && nx < width as i32
                && ny < height as i32
                && grid[ni as usize] == c
            {
                if !seen[ni as usize] {
                    seen[ni as usize] = true;
                    queue.push_back((nx, ny));
                }
            } else if d.1 == 0 {
                vertical_sides.push((y, x * 4 + d.0));
            } else {
                horizonzal_sides.push((x, y * 4 + d.1));
            }
        }
    }

    Region {
        plant: plant.to_string(),
        area,
        horizonzal_sides,
        vertical_sides,
        hsl: 0,
        vsl: 0,
        sides: 0,
    }
}

fn remove_connected(s: (i32, i32), sides: &mut Vec<(i32, i32)>) {
    // since there's always only a very small number of side tiles, it's
    // faster to use a Vec instead of a HashSet or a BinaryHeap
    let mut a = s.0 + 1;
    while let Some(k) = sides.iter().position(|p| p.0 == a && p.1 == s.1) {
        sides.swap_remove(k);
        a += 1;
    }
    let mut a = s.0 - 1;
    while let Some(k) = sides.iter().position(|p| p.0 == a && p.1 == s.1) {
        sides.swap_remove(k);
        a -= 1;
    }
}

pub fn solution() {
    // let input = fs::read_to_string("./inputs/puzzle_12/input.txt").expect("Could not read file");
    let input =
        fs::read_to_string("../inputs/puzzle_12/input.sample.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    let mut seen = vec![false; grid.len()];
    let mut total1 = 0;
    let mut total2 = 0;
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            if seen[y as usize * width + x as usize] {
                continue;
            }

            let mut debug_on = false;
            let c = grid[y as usize * width + x as usize];
            if c == b'X' {
                debug_on = true;
            }

            // Fill region. This will give us its area and all its horizontal
            // and vertical side tiles.
            let mut region = fill_region(x, y, &grid, width, height, &mut seen);

            let hsl = region.horizonzal_sides.clone().len();
            let vsl = region.vertical_sides.clone().len();
            let hs = region.horizonzal_sides.clone();
            let vs = region.vertical_sides.clone();

            total1 += region.area * (hsl + vsl);

            // find connected side tiles and count how many sides there are
            let mut n_sides = 0;
            for sides in [region.horizonzal_sides, region.vertical_sides].iter_mut() {
                while !sides.is_empty() {
                    let s = sides.swap_remove(0);
                    remove_connected(s, sides);
                    n_sides += 1;
                }
                if debug_on {
                    dbg!(n_sides);
                }
            }

            let r = Region {
                plant: region.plant,
                area: region.area,
                sides: n_sides,
                horizonzal_sides: hs,
                vertical_sides: vs,
                hsl: hsl,
                vsl: vsl,
            };

            if debug_on {
                // dbg!(&r);
                let json = serde_json::to_string(&r).unwrap();
                std::fs::write(format!("./looking_at_sides.json"), json)
                    .expect("Failed to write JSON file");
            }

            total2 += region.area * n_sides;
        }
    }

    println!("{}", total1);
    println!("{}", total2);
}

// use std::collections::HashSet;
// use std::fs::File;
// use std::io::{self, BufRead};

// use petgraph::adj::NodeIndex;
// use petgraph::algo;
// use petgraph::prelude::UnGraphMap;
// use petgraph::visit::IntoNodeReferences;

// pub fn solution() {
//     let aoc_2024_12 = File::open("./inputs/puzzle_12/input.txt").expect("Failed to open file");
//     let grid: Vec<Vec<char>> = io::BufReader::new(aoc_2024_12)
//         .lines()
//         .filter_map(|line| line.ok())
//         .map(|line| line.chars().collect())
//         .collect();

//     let rows = grid.len() as i32;
//     let cols = grid[0].len() as i32;

//     let mut visited = HashSet::new();
//     let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

//     let mut total_sum = 0;

//     for r in 0..rows {
//         for c in 0..cols {
//             if visited.contains(&(r, c)) {
//                 continue;
//             }

//             let mut queue = vec![(r, c)];
//             visited.insert((r, c));

//             let mut group_size = 0;
//             let mut perimeter = 0;

//             while let Some((x, y)) = queue.pop() {
//                 let value = grid[x as usize][y as usize];
//                 let mut local_perimeter = 4;
//                 group_size += 1;

//                 for &(dx, dy) in &directions {
//                     let nx = x + dx;
//                     let ny = y + dy;

//                     if nx >= 0 && nx < rows && ny >= 0 && ny < cols {
//                         if grid[nx as usize][ny as usize] == value {
//                             if visited.insert((nx, ny)) {
//                                 queue.push((nx, ny));
//                             }
//                             local_perimeter -= 1;
//                         }
//                     }
//                 }
//                 perimeter += local_perimeter;
//             }
//             total_sum += group_size * perimeter;
//         }
//     }
//     println!("Result: {}", total_sum);

//     // B.
//     let mut graph = UnGraphMap::<(i32, i32), ()>::new();

//     for y in 0..rows {
//         for x in 0..cols {
//             // Add node to the graph
//             let node = graph.add_node((x, y));
//             let c = grid[y as usize][x as usize];

//             // Add edges to the graph
//             for &(dx, dy) in &directions {
//                 let nx = x + dx;
//                 let ny = y + dy;

//                 // Check if the node is within the grid
//                 if nx >= 0 && nx < cols && ny >= 0 && ny < rows {
//                     if grid[ny as usize][nx as usize] == c {
//                         graph.add_edge(node, (nx, ny), ());
//                     }
//                 }
//             }
//         }
//     }

//     let graph = algo::condensation(graph.into_graph::<NodeIndex>(), false);
//     let sum = graph.node_references().map(|(_, nodes)| {
//         let id = grid[nodes[0].1 as usize][nodes[0].0 as usize];
//         let node_size = nodes.len();
//         let perimeter = nodes
//             .iter()
//             .map(|n| {
//                 let mut count = 0;

//                 for i in 0..4 {
//                     let (dx, dy) = directions[i];
//                     let (dx1, dy1) = directions[(i + 1) % 4];

//                     let first_neighbor_same_region = {
//                         let nx = n.0 + dx;
//                         let ny = n.1 + dy;
//                         nx >= 0
//                             && ny >= 0
//                             && (ny as usize) < grid.len()
//                             && (nx as usize) < grid[ny as usize].len()
//                             && grid[ny as usize][nx as usize] == id
//                     };

//                     let second_neighbor_same_region = {
//                         let nx = n.0 + dx1;
//                         let ny = n.1 + dy1;
//                         nx >= 0
//                             && ny >= 0
//                             && (ny as usize) < grid.len()
//                             && (nx as usize) < grid[ny as usize].len()
//                             && grid[ny as usize][nx as usize] == id
//                     };

//                     let diagonal_test = {
//                         let nx = n.0 + dx + dx1;
//                         let ny = n.1 + dy + dy1;
//                         nx >= 0
//                             && ny >= 0
//                             && (ny as usize) < grid.len()
//                             && (nx as usize) < grid[ny as usize].len()
//                             && grid[ny as usize][nx as usize] != id
//                     };

//                     if first_neighbor_same_region && second_neighbor_same_region && diagonal_test {
//                         count += 1;
//                     } else if !first_neighbor_same_region && !second_neighbor_same_region {
//                         count += 1;
//                     }
//                 }

//                 count
//             })
//             .sum::<usize>();

//         node_size * perimeter
//     });

//     println!("Result: {}", sum.sum::<usize>());
// }
