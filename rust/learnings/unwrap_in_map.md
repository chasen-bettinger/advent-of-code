In rust, should you unwrap inside an iter function?

====

https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect

the exact resource that I was looking for

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}

///////

https://corrode.dev/blog/rust-option-handling-best-practices/
For most cases, I prefer this syntax:

let Some(value) = some_function() else {
    return Err("Descriptive error message".into());
};

To me, let-else is the best solution for handling None because:

    It’s part of the standard library.
    It works for both libraries and applications.
    It’s easy to understand for beginners.
    It’s reasonably compact.
    It allows for more complex error handling logic in the else block if needed.
    Learning the mechanics behind it is helpful in other places in Rust.

///////

https://doc.rust-lang.org/std/option/index.html#the-question-mark-operator-

Similar to the Result type, when writing code that calls many functions that return the Option type, handling Some/None can be tedious. The question mark operator, ?, hides some of the boilerplate of propagating values up the call stack.

It replaces this:

fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    let a = stack.pop();
    let b = stack.pop();

    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}

With this:

fn add_last_numbers(stack: &mut Vec<i32>) -> Option<i32> {
    Some(stack.pop()? + stack.pop()?)
}

///////

https://www.reddit.com/r/rust/comments/62hi8e/best_way_to_deal_with_iteratormap_when_map/

asks the same question that I do.. doesn't appear to have a good answer for my case

No, it's generally not recommended to use `unwrap()` inside iterators as it can cause panics. A better approach would be to use `filter_map` which allows us to safely handle potential `None` values or errors. Here's the safer version:

```rust:src/main.rs
    let matches: Vec<(i32, i32)> = re
        .captures_iter(memory)
        .filter_map(|x| {
            let a = x.name("a")?.as_str().parse::<i32>().ok()?;
            let b = x.name("b")?.as_str().parse::<i32>().ok()?;
            Some((a, b))
        })
        .collect();
```

This version:
1. Uses `filter_map` instead of `map`
2. Uses the `?` operator with `Option` (which is valid in this context)
3. Safely handles both the regex capture group extraction and the parsing
4. Will skip any items that fail to parse instead of panicking

This is more idiomatic Rust as it handles potential failures gracefully without risk of runtime panics.
