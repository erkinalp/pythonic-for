# Pythonic For

[![Crates.io](https://img.shields.io/crates/v/pythonic-for.svg)](https://crates.io/crates/pythonic-for)
[![Documentation](https://docs.rs/pythonic-for/badge.svg)](https://docs.rs/pythonic-for)
[![License](https://img.shields.io/crates/l/pythonic-for.svg)](https://github.com/erkinalp/pythonic-for/blob/default/LICENSE-MIT)

A Rust crate that provides a Python-style `for` loop with an optional `else` clause.

## Features

- **Optional Else Clause**: The else clause is executed if the loop completes without a break or an error, similar to Python's for-else construct. The else clause is optional.
- **Inclusive/Exclusive Ranges**: Supports both inclusive (`..=`) and exclusive (`..`) ranges.
- **Step Values**: Allows specifying a step value for iteration, including negative steps for reverse iteration.
- **Error Handling**: Handles Rust errors similarly to how Python handles exceptions in for loops. If an error occurs during iteration, the else clause is not executed.
- **Collection Support**: Works with various Rust collections (Vec, HashMap, HashSet, etc.) and custom iterators.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pythonic-for = "0.1.0"
```

## Usage

### Basic Usage Without Else Clause

```rust
use pythonic_for::pythonic_for;

// Simple for loop without else clause
let mut sum = 0;
pythonic_for!((i in 0..5) {
    sum += i;
});
assert_eq!(sum, 10); // 0+1+2+3+4 = 10

// For loop with step value without else clause
let mut sum = 0;
pythonic_for!((i in 0..10, step = 2) {
    sum += i;
});
assert_eq!(sum, 20); // 0+2+4+6+8 = 20
```

### Usage With Else Clause

```rust
use pythonic_for::pythonic_for;

// Basic for-else loop
let mut found = false;
pythonic_for!((i in 0..5) {
    if i == 10 {
        found = true;
        break;
    }
} else {
    found = false;
});
assert_eq!(found, false);

// For loop with break
let mut found = false;
pythonic_for!((i in 0..5) {
    if i == 3 {
        found = true;
        break;
    }
} else {
    found = false;
});
assert_eq!(found, true);

// Inclusive range
let mut sum = 0;
pythonic_for!((i in 1..=5) {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115

// Step value
let mut sum = 0;
pythonic_for!((i in 0..10, step = 2) {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 120); // 0+2+4+6+8+100 = 120

// Negative step
let mut sum = 0;
pythonic_for!((i in 10..0, step = -2) {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130

// Iterating over a collection
let vec = vec![1, 2, 3, 4, 5];
let mut sum = 0;
pythonic_for!((i in vec) {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
```

### Custom Iterators

The macro works with custom iterators as well:

```rust
struct SquareIter {
    current: i32,
    end: i32,
}

impl Iterator for SquareIter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let result = self.current * self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

let square_iter = SquareIter { current: 1, end: 3 };
let mut sum = 0;

pythonic_for!((value in square_iter) {
    sum += value;
} else {
    sum += 100;
});
assert_eq!(sum, 114); // 1+4+9+100 = 114
```

## Break Statements

The `pythonic_for!` macro now automatically handles break statements without requiring any manual intervention:

```rust
pythonic_for!((i in 0..5) {
    if i == 3 {
        break; // Automatically sets _break_occurred internally
    }
} else {
    // This will not execute if break was used
});
```

## Common Pitfalls and Best Practices

### Infinite Iterators

When using infinite iterators like `cycle()` with an else clause, the else clause becomes unreachable unless you include a break statement:

```rust
// LOGICAL ERROR: The else clause will never execute
pythonic_for!((n in numbers.iter().cycle()) {
    // This will cycle through the collection indefinitely
} else {
    // This code is unreachable without a break in the loop body
});

// CORRECT: Use a break condition with infinite iterators
pythonic_for!((n in numbers.iter().cycle()) {
    // Process n
    if some_condition {
        break; // Exit condition makes the loop finite
    }
} else {
    // Now this can be reached if the break condition is met
});
```

### Nested Loops

When using nested loops, only breaks in the outermost loop affect the else clause:

```rust
pythonic_for!((i in 0..3) {
    for j in 0..3 {
        if j == 1 {
            break; // This only breaks from the inner loop
        }
    }
} else {
    // This will still execute since the break was in the inner loop
});
```

### Error Handling

For robust error handling, combine the pythonic_for macro with Result types:

```rust
let mut result: Result<i32, &str> = Ok(0);
pythonic_for!((i in 0..5) {
    match process(i) {
        Ok(value) => { /* continue processing */ },
        Err(e) => {
            result = Err(e);
            break; // Exit the loop on error
        }
    }
} else {
    // This only executes if no errors occurred
});
```

## Iterator Adapters

The `pythonic_for!` macro works seamlessly with all standard Iterator adapters:

```rust
// Using enumerate
let letters = vec!['a', 'b', 'c'];
pythonic_for!((pair in letters.iter().enumerate()) {
    let (idx, ch) = pair;
    println!("Index: {}, Value: {}", idx, ch);
});

// Using take to limit iterations
let numbers = vec![1, 2, 3, 4, 5, 6];
pythonic_for!((n in numbers.iter().take(3)) {
    // Only iterates over the first 3 elements
});

// Using skip to start from a specific position
pythonic_for!((n in numbers.iter().skip(2)) {
    // Skips the first 2 elements
});

// Using flat_map for nested collections
let nested = vec![vec![1, 2], vec![3, 4]];
pythonic_for!((n in nested.iter().flat_map(|v| v.iter())) {
    // Flattens the nested structure
});

// Using cycle with break for infinite iteration
pythonic_for!((n in numbers.iter().cycle()) {
    // Will cycle through the collection indefinitely
    if some_condition {
        break; // Use break to exit the loop
    }
});

// Other adapters like filter_map, chain, zip, etc. are also supported
```

## Error Handling

The `pythonic_for!` macro supports two types of error handling:

1. **Panic handling**: If a panic occurs in the loop body, the else clause is skipped.

2. **Result-based error handling**: You can use Result types with the macro:

```rust
let mut result: Result<i32, &str> = Ok(0);
pythonic_for!((i in 0..5) {
    if some_condition {
        result = Err("Error occurred");
        break;
    }
    // Process if Ok
    if let Ok(val) = result {
        result = Ok(val + i);
    }
} else {
    // This only executes if no error occurred and no break was used
});
```

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
