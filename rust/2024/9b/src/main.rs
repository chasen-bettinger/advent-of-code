use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct SpaceBlock {
    quantity: i64,
    index: usize,
}

struct Output {
    sum: i64,
}

fn y(input: &str) -> Output {
    let c = input.chars().collect::<Vec<char>>();
    let mut id_number: i64 = 0;
    let mut is_file_block = true;
    let mut blocks: Vec<String> = vec![];
    let mut block_quantity: HashMap<i64, SpaceBlock> = HashMap::new();
    c.iter().for_each(|x| {
        let Some(v) = x.to_digit(10) else {
            return;
        };
        let mut block: String = id_number.to_string();
        if !is_file_block {
            block = ".".to_string();
        } else {
            block_quantity.insert(
                id_number,
                SpaceBlock {
                    quantity: v.into(),
                    index: blocks.len(),
                },
            );
        }
        for _ in 0..v {
            blocks.push(block.clone());
        }
        is_file_block = !is_file_block;
        if is_file_block {
            id_number += 1;
        }
    });

    let mut sorted_keys: Vec<i64> = block_quantity.keys().cloned().collect();
    // look into why the error happened when sorting with strings
    sorted_keys.sort_by(|a, b| b.cmp(a));
    for key in sorted_keys {
        let block_q = block_quantity.get(&key).unwrap();
        let mut in_free_space = false;
        let mut free_space_index = 0;
        let mut free_space_quantity = 0;

        for i in 0..blocks.len() {
            let v = &blocks[i];
            in_free_space = v == ".";
            if in_free_space {
                if free_space_index == 0 {
                    free_space_index = i;
                }
                free_space_quantity += 1;
                continue;
            }

            if free_space_index == 0 && free_space_quantity == 0 {
                continue;
            }

            if free_space_index >= block_q.index {
                break;
            }

            if free_space_quantity >= block_q.quantity {
                for j in 0..block_q.quantity {
                    blocks[free_space_index + j as usize] = key.to_string();
                    blocks[block_q.index + j as usize] = ".".to_string();
                }

                break;
            }

            free_space_index = 0;
            free_space_quantity = 0;
        }
    }

    let mut sum: i64 = 0;
    for i in 0..blocks.len() {
        if let Ok(v) = blocks[i].parse::<i64>() {
            sum += v * i as i64;
        }
    }

    Output { sum }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "2333133121414131402";
        assert_eq!(y(input).sum, 2858);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!(y(input).sum, 6311837662089);
    }
}
