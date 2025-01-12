use std::collections::HashMap;
use std::error::Error;
use std::{fs, io::BufRead};

fn read_file() -> std::io::Result<String> {
    fs::read_to_string("input.txt")
}

//fn read_file() -> std::io::Result<()> {
//    let file = std::fs::File::open("input.txt")?;
//    let reader = std::io::BufReader::new(file);
//    for line in reader.lines() {
//        println!("{:?}", line);
//    }
//    Ok(())
//}

fn parse_input() -> (Vec<i32>, HashMap<i32, i32>) {
    let output = read_file().unwrap();
    let mut left: Vec<_> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    output.split("\n").for_each(|line| {
        // println!("{:?}", line)
        let mut split_line = line.split("   ");
        let Some(c_left) = split_line.next() else {
            return;
        };

        let Ok(y): Result<i32, _> = c_left.parse() else {
            return;
        };

        let Some(c_right) = split_line.next() else {
            return;
        };

        let Ok(x): Result<i32, _> = c_right.parse() else {
            return;
        };

        left.push(y);

        let count = right.get(&x).unwrap_or(&0);
        right.insert(x, count + 1);
    });

    left.sort();

    (left, right)
}

fn hello(left: &Vec<i32>, right: &HashMap<i32, i32>) -> Result<i32, Box<dyn Error>> {
    let mut similarity_score = 0;

    for i in 0..left.len() {
        let left_value = left.get(i).unwrap();
        let right_value = right.get(left_value).unwrap_or(&0);
        //println!("lv: {:?}", left_value);
        //println!("rv: {:?}", right_value);
        // let difference = left_value - right_value;
        let frequency = left_value * right_value;
        similarity_score += frequency;
    }

    println!("{:?}", similarity_score);
    Ok(similarity_score)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn simple_case() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = HashMap::from([(3, 3), (4, 1)]);
        assert_eq!(hello(&left, &right).unwrap(), 31);
    }

    #[test]
    fn input_case() {
        let input = parse_input();
        assert_eq!(hello(&input.0, &input.1).unwrap(), 31);
    }
}
