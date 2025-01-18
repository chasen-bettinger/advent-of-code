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

fn get_frequency_of_xmas(input: &str) -> i32 {
    0
}
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
