fn main() {
    println!("Hello, world!");
}

fn y(input: &str) -> i64 {
    let c = input.chars().collect::<Vec<char>>();
    let mut id_number = 0;
    let mut is_file_block = true;
    let mut blocks: Vec<String> = vec![];
    let mut count_of_free_space = 0;
    c.iter().for_each(|x| {
        let Some(v) = x.to_digit(10) else {
            return;
        };
        let mut block: String = id_number.to_string();
        if !is_file_block {
            block = ".".to_string();
        }
        for _ in 0..v {
            blocks.push(block.clone());
            if !is_file_block {
                count_of_free_space += 1;
            }
        }
        is_file_block = !is_file_block;
        if is_file_block {
            id_number += 1;
        }
    });

    let rev_blocks = blocks.clone().into_iter().rev().collect::<Vec<String>>();
    for i in 0..rev_blocks.len() {
        let Ok(s) = rev_blocks[i].parse::<i32>() else {
            continue;
        };
        for j in 0..blocks.len() {
            let w = &blocks[j];
            if *w != "." {
                continue;
            }
            blocks[j] = s.to_string();
            break;
        }
    }

    let k = blocks[0..blocks.len() - count_of_free_space].to_vec();

    let mut multiplier = 0;
    let mut sum: i64 = 0;
    for i in 0..k.len() {
        if let Ok(v) = k[i].parse::<i64>() {
            sum += v * multiplier;
            multiplier += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "2333133121414131402";
        assert_eq!(y(input), 1928);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        assert_eq!(y(input), 6288707484810);
    }
}
