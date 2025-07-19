I'll explain when `lazy_static` is particularly useful in Rust. Here are the key scenarios:

1. **Expensive Computations**
```rust
lazy_static! {
    static ref COMPLEX_CALCULATION: Vec<u64> = {
        // This expensive calculation only happens once, when first accessed
        let mut numbers = Vec::new();
        for i in 0..1000 {
            numbers.push(i * i);
        }
        numbers
    };
}
```

2. **Global State That Requires Heap Allocation**
```rust
use std::collections::HashMap;

lazy_static! {
    static ref GLOBAL_CONFIG: HashMap<&'static str, String> = {
        let mut map = HashMap::new();
        map.insert("api_key", std::env::var("API_KEY").unwrap_or_default());
        map.insert("base_url", "https://api.example.com".to_string());
        map
    };
}
```

Key benefits and use cases:

1. **Runtime Initialization**
   - Regular `static` variables must be initialized at compile time
   - `lazy_static` allows initialization with runtime values (like environment variables or file contents)

2. **Memory Efficiency**
   - The static is only initialized when first accessed
   - If your program never uses the static, it never gets initialized

3. **Thread Safety**
   - `lazy_static` ensures thread-safe initialization
   - Perfect for global caches or configuration that needs to be accessed from multiple threads

4. **Common Use Cases**:
   - Logger configurations
   - Application settings
   - Regular expression patterns
   - Database connection pools
   - Global caches

Example with regex (very common use case):
```rust
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EMAIL_RE: Regex = Regex::new(r"^\w+@\w+\.\w+$").unwrap();
}

fn is_valid_email(email: &str) -> bool {
    EMAIL_RE.is_match(email)  // Regex is compiled only once, when first used
}
```

Without `lazy_static`, you'd either need to:
- Recompile the regex every time (expensive)
- Use a regular `static` (but Regex can't be created at compile time)
- Use a mutex or other synchronization primitive manually

The macro essentially gives you the best of both worlds: the convenience of global state with the flexibility of runtime initialization, all while maintaining thread safety.
