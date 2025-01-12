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

fn parse_input() -> [Vec<i32>; 2] {
    let output = read_file().unwrap();
    let mut left: Vec<_> = Vec::new();
    let mut right: Vec<_> = Vec::new();
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
        right.push(x);
    });

    left.sort();
    right.sort();

    [left, right]
}

fn hello(left: &Vec<i32>, right: &Vec<i32>) -> Result<i32, Box<dyn Error>> {
    let mut total_distance = 0;

    for i in 0..left.len() {
        let left_value = left.get(i).unwrap();
        let right_value = right.get(i).unwrap();
        //println!("lv: {:?}", left_value);
        //println!("rv: {:?}", right_value);
        // let difference = left_value - right_value;
        let difference = (right_value - left_value).abs();
        total_distance += difference;
    }

    println!("{:?}", total_distance);
    Ok(total_distance)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let left = vec![3, 4, 2, 1, 3];
        let right = vec![4, 3, 5, 3, 9];
        assert_eq!(hello(&left, &right).unwrap(), 11);
    }

    #[test]
    fn input_case() {
        let input = parse_input();
        let [left, right] = input;
        assert_eq!(hello(&left, &right).unwrap(), 11);
    }
}
