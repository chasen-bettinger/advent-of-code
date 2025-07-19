use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

/*
1,0 -> right
-1,0 -> left
0,1 -> down
0,-1 -> up
*/

struct Garden {
    plants: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Plant {
    character: String,
    area: i64,
    horizontal_sides: Vec<(i32, i32)>,
    sides: i64,
    vertical_sides: Vec<(i32, i32)>,
}

impl Garden {
    fn new(input: &str) -> Self {
        let plants: Vec<Vec<String>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_string()).collect())
            .collect();
        Self { plants }
    }

    fn find_sides(&self, entry: (i32, i32), sides: &mut Vec<(i32, i32)>) {
        let mut a = entry.0 + 1;
        while let Some(pos) = sides.iter().position(|s| s.0 == a && s.1 == entry.1) {
            sides.swap_remove(pos);
            a += 1;
        }
        let mut a = entry.0 - 1;
        while let Some(pos) = sides.iter().position(|s| s.0 == a && s.1 == entry.1) {
            sides.swap_remove(pos);
            a -= 1;
        }
    }

    fn get_plant(
        &mut self,
        sx: i32,
        sy: i32,
        grid: &[u8],
        width: usize,
        height: usize,
        seen: &mut [bool],
    ) -> Plant {
        let plant_value = self.plants[sy as usize][sx as usize].clone();
        if plant_value == "B" {
            println!("hello_world");
        }
        let mut area = 0;
        let mut horizontal_sides = Vec::new();
        let mut vertical_sides = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back((sx, sy));
        seen[sy as usize * width + sx as usize] = true;
        let v = grid[sy as usize * width + sx as usize];

        while let Some((x, y)) = queue.pop_front() {
            area += 1;

            for d in DIRECTIONS {
                // x is treated as the column
                let nx = x + d.0;
                // y is treated as the row
                let ny = y + d.1;
                // in a 1d grid, the row becomes multiplied by the "number of items per row" (width)
                let ni = ny * width as i32 + nx;

                let index_not_negative = nx >= 0 && ny >= 0;

                if index_not_negative
                    && ny < height as i32
                    && nx < width as i32
                    && grid[ni as usize] == v
                {
                    if !seen[ni as usize] {
                        seen[ni as usize] = true;
                        queue.push_back((nx, ny));
                    }
                }
                // because d.1 == 0 means we're moving laterally, we know that if the item is not the target, then it's a vertical side.
                else if d.1 == 0 {
                    vertical_sides.push((y, x * 4 + d.0));
                } else {
                    horizontal_sides.push((x, y * 4 + d.1));
                }
            }
        }

        return Plant {
            character: plant_value,
            area,
            horizontal_sides,
            vertical_sides,
            sides: 0,
        };
    }

    fn get_plants(&mut self) -> Vec<Plant> {
        // transform the 2d grid into a 1d grid so that sides can be
        //  accurately counted
        let grid = self
            .plants
            .iter()
            .flatten()
            .flat_map(|l| l.as_bytes())
            .copied()
            .collect::<Vec<_>>();

        let height = self.plants.len();
        let width = self.plants[0].len();
        let mut seen = vec![false; height * width];
        let mut plants = Vec::<Plant>::new();
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let idx = y * width as i32 + x;
                if seen[idx as usize] {
                    continue;
                }

                let mut plant = self.get_plant(x, y, &grid, width, height, &mut seen);

                let mut n_sides = 0;
                for sides in
                    [plant.horizontal_sides.clone(), plant.vertical_sides.clone()].iter_mut()
                {
                    while !sides.is_empty() {
                        let s = sides.swap_remove(0);
                        self.find_sides(s, sides);
                        n_sides += 1;
                    }
                }

                plant.sides = n_sides;

                plants.push(plant);
            }
        }
        return plants;
    }

    fn get_fence_cost(&mut self) -> i64 {
        let plants = self.get_plants();
        let mut fence_cost = 0;
        for plant in plants {
            fence_cost +=
                plant.area * (plant.horizontal_sides.len() + plant.vertical_sides.len()) as i64;
        }
        fence_cost
    }

    fn get_fence_cost_with_sides(&mut self) -> i64 {
        let plants = self.get_plants();
        let mut fence_cost = 0;
        for plant in plants {
            fence_cost += plant.area * plant.sides as i64;
        }
        fence_cost
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        let mut garden = Garden::new(input);
        let output = garden.get_fence_cost();
        assert_eq!(output, 140);
    }

    #[test]
    fn test_basic_b() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        let mut garden = Garden::new(input);
        let output = garden.get_fence_cost();
        assert_eq!(output, 1930);
    }

    #[test]
    fn test_input_a() {
        let input = include_str!("../inputs/puzzle_12/input.txt");
        let mut garden = Garden::new(input);
        let output = garden.get_fence_cost();
        assert_eq!(output, 1518548);
    }

    #[test]
    fn test_basic_edge() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        let mut garden = Garden::new(input);
        let output = garden.get_fence_cost_with_sides();
        assert_eq!(output, 80);
    }

    #[test]
    fn test_basic_b_edge() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        let mut garden = Garden::new(input);
        let output = garden.get_fence_cost_with_sides();
        assert_eq!(output, 1206);
    }

    #[test]
    fn test_input_b() {
        let input = include_str!("../inputs/puzzle_12/input.txt");
        let mut garden = Garden::new(input);
        let output = garden.get_fence_cost_with_sides();
        assert_eq!(output, 909564);
    }

}
