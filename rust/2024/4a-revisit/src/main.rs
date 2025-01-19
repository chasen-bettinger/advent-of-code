use array_util::try_from_fn;

fn main() {
    println!("Hello, world!");
}

/*
Source: https://github.com/ephemient/aoc2024/blob/main/rs/src/day4.rs

This design looks at bytes of the target values,
not actual characters. It uses specific directional values,
excluding left operations because the targets include both
the target and the target's reverse.
It heavily leverages the iterator trait to parse the input,
wrapping callbacks in filter_map + try_from_fn to capture
errors.
It collects everything and waits until the end to filter
for the target values.
*/

fn get_frequency_of_xmas_pattern_a(input: &str) -> i32 {
    // b transforms the string to a byte array
    // it is dereferenced because the program
    // needs the literal values and not references
    // to those values
    const XMAS: [[u8; 4]; 2] = [*b"XMAS", *b"SAMX"];

    // i don't understand the [..] syntax
    /*
        The `[..]` syntax is equivalent to writing `[0..len]`, where
        `len` is the length of the vector. It's a way to create a
        slice that contains all elements of the collection.
    */
    let lines = &input.lines().collect::<Vec<_>>()[..];
    lines
        .iter() // iter here so that you gain access to enumerate
        .enumerate() // enumerate so that you get access to index + value
        .flat_map(|(y, line)| {
            // y is rows, line is the actual content
            /*
            The `move` keyword is necessary here because of Rust's closure ownership rules. In this case, `move` is telling the closure to take ownership of any variables it captures from its environment, rather than borrowing them.

            Let's break down why it's needed in this specific context:

            1. This closure is inside another closure that has captured `y` and `line`
            2. The outer closure is used in a `flat_map` which may live longer than the current iteration
            3. The `move` ensures that `x` and any captured variables (like `y` and `line`) are moved into each closure instance, making them independent of the original scope

            Without `move`, you might get compiler errors because:
            - The closure might outlive the borrowed values
            - Multiple iterations might try to access the same borrowed values simultaneously

            Here's a simplified example to illustrate:
            ```rust
            let lines = vec!["abc", "def"];
            lines.iter().enumerate().flat_map(|(y, line)| {
                // This closure needs 'move' because 'y' and 'line' need to be valid
                // for each iteration of the flat_map
                (0..line.len()).flat_map(move |x| {
                    // Values from outer scope are now owned by this closure
                    Some((x, y, line))
                })
            });
            ```

            The `move` keyword is particularly important in this code because:
            1. The closure is used in a `flat_map` which creates an iterator
            2. The iterator might be processed lazily
            3. Multiple iterations might happen concurrently
            4. Each iteration needs its own independent copy of the captured variables
            */
            (0..line.len()).flat_map(move |x| {
                // iterate again so that you access x, unclear: move
                // flat map is because we want a single iterable at the end
                // down, down-right, right, up-right
                [(1, 0), (1, 1), (0, 1), (-1, 1)] // directions
                    .iter() // iter so you gain access to iterator traits
                    .filter_map(move |(dx, dy)| {
                        // filter_map so that you can map over the iterator and filter out None
                        // i don't really understand why try_from_fn is used
                        // but I think it's because you can iterate and not
                        // worry about an error
                        try_from_fn(|i| {
                            lines
                                .get(y + i * dy)? // this collects the row
                                .as_bytes() // this converts the row to bytes
                                .get(x.wrapping_add_signed(i as isize * dx)) // this gets the value at the index
                                .copied() // maps a &T to a T by copying the value
                        })
                    })
            })
        })
        .filter(|s| XMAS.contains(s))
        .count()
        .try_into()
        .unwrap_or(0)
}

/* https://github.com/AxlLind/AdventOfCode/blob/main/2024/src/bin/04.rs

I like this pattern for a couple of reasons:
1. it leverages match.. I think that match is effective at communicating
and a way to handle variable cases.
2. it's makes effective use of space, prefering inline evaluations like
b"XMAS"[i as usize] instead of a function call. and a getter function that
handles errors well.
*/
fn get_frequency_of_xmas_pattern_b(input: &str) -> i32 {
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let rows: i32 = lines.len().try_into().unwrap();
    let cols: i32 = lines[0].len().try_into().unwrap();

    let mut frequency = 0;
    for r in 0..rows {
        for c in 0..cols {
            match lines[r as usize][c as usize] {
                b'X' => {
                    frequency += chasen_find_xmas(&lines, r, c);
                    // frequency += find_xmas(&lines, r, c);
                }
                _ => {}
            }
        }
    }
    frequency.try_into().unwrap_or(0)
}

fn get_frequency_of_xmas(input: &str) -> i32 {
    get_frequency_of_xmas_pattern_b(&input)
}

// not sure how this works given that there is a
// an overflow error when summing because of the usize
// with negative values
// this is the original find_xmas function
fn find_xmas(m: &[&[u8]], r: usize, c: usize) -> usize {
    [
        (0, -1),
        (-1, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ]
    .iter()
    .filter(|(dr, dc)| {
        (1..4).all(|i| {
            let (rr, cc) = (r + (dr * i) as usize, c + (dc * i) as usize);
            get(m, rr, cc) == b"XMAS"[i as usize]
        })
    })
    .count()
}

// this is an alteration of the find_xmas function
// but it uses i32 values. I'm not really sure how type
// inference works, shifting all values in the [..] into
// i32 / usize values based on what's inside the all
// closure.
fn chasen_find_xmas(m: &[&[u8]], r: i32, c: i32) -> usize {
    [
        (0, -1),
        (-1, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ]
    .iter()
    .filter(|(dr, dc)| {
        (1..4).all(|i| {
            let a_r = r + (dr * i);
            let a_c = c + (dc * i);
            if a_r < 0 || a_c < 0 || a_r >= m.len() as i32 || a_c >= m[a_r as usize].len() as i32 {
                return false;
            }
            get(m, a_r as usize, a_c as usize) == b"XMAS"[i as usize]
        })
    })
    .count()
}

// a safe getter for retrieving a value from a 2d matrix
fn get(m: &[&[u8]], r: usize, c: usize) -> u8 {
    *m.get(r).and_then(|row| row.get(c)).unwrap_or(&b'_')
}
/* most-similar implementation to mine: https://github.com/rust-dd/aoc-2024 */
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_simple() {
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let frequency = get_frequency_of_xmas(input);
        assert_eq!(frequency, 18);
    }
    #[test]
    fn test_input() {
        let input = fs::read_to_string("input.txt").unwrap();
        let frequency = get_frequency_of_xmas(&input);
        assert_eq!(frequency, 2633);
    }
}
