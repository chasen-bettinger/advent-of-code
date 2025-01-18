fn get_frequency_of_xmas(input: &str) -> i32 {
    const XMAS: [[u8; 4]; 2] = [*b"XMAS", *b"SAMX"];

    // i don't understand the [..] syntax
    let lines = &input.lines().collect::<Vec<_>>()[..];
    lines
        .iter() // iter here so that you gain access to enumerate
        .enumerate() // enumerate so that you get access to index + value
        .flat_map(|(y, line)| {
            // y is rows, line is the actual content
            (0..line.len()).flat_map(move |x| {
                // iterate again so that you access x, unclear: flat_map and move
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
                                .get(x + i * dx) // this gets the value at the index
                                                 // .copied() // this converts the value to an option
                        })
                    })
            })
        })
        .filter(|s| XMAS.contains(s))
        .count()
        .try_into()
        .unwrap_or(0)
}
