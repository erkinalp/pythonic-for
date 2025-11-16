# Pythonic For

[![Crates.io](https://img.shields.io/crates/v/pythonic-for.svg)](https://crates.io/crates/pythonic-for)
[![Documentation](https://docs.rs/pythonic-for/badge.svg)](https://docs.rs/pythonic-for)
[![License](https://img.shields.io/crates/l/pythonic-for.svg)](https://github.com/erkinalp/pythonic-for/blob/default/LICENSE-MIT)

A Rust crate that provides Python-style `for` and `while` loops with an optional `else` clause.

## Features

- **Optional Else Clause**: For both `for` and `while` loops, the `else` clause is executed if the loop completes without a `break` statement and without encountering a panic.
- **`for` loop features**:
    - **Inclusive/Exclusive Ranges**: Supports both inclusive (`..=`) and exclusive (`..`) ranges.
    - **Step Values**: Allows specifying a step value for iteration, including negative steps for reverse iteration.
    - **Collection Support**: Works with various Rust collections (Vec, HashMap, HashSet, etc.) and custom iterators.
- **`while` loop features**:
    - **Standard `while` and `do-while` variants**: Supports both common `while` loop forms.
- **Error Handling**: Consistent panic handling for both loop types (else clause skipped).
- **Break Statement Handling**: `break` statements within `pythonic_for!` or `pythonic_while!` bodies correctly prevent their respective `else` clauses from executing.

## Semantics (Python-compatible)

This crate implements Python's for-else and while-else semantics as documented in the [Python Language Reference](https://docs.python.org/3/reference/compound_stmts.html#the-for-statement):

> In a `for` or `while` loop the `break` statement may be paired with an `else` clause. If the loop finishes without executing the `break`, the `else` clause executes.

Specifically, the `else` (or `final`) clause executes if and only if:
- The loop completes all iterations naturally (including zero iterations), AND
- No `break` statement is executed, AND
- No panic occurs in the loop body

The `else` clause does NOT execute if:
- A `break` statement is executed (including labeled breaks that exit the pythonic loop)
- A panic occurs in the loop body
- A `return` statement exits the enclosing function

### Truth Table

| Condition | Else Executes? |
|-----------|----------------|
| Loop completes normally (0+ iterations, no break, no panic) | ✓ Yes |
| Loop never iterates (empty range/iterator) but no break | ✓ Yes |
| `break` is executed | ✗ No |
| Panic occurs in loop body | ✗ No |
| `return` exits function | ✗ No |

### Contrast with Other Languages

Some languages (PHP, Jinja, Twig) have proposed or implemented "for-else" where `else` runs only if the loop **never iterated** (zero iterations). This crate does **not** follow that semantic. Our `else` clause runs whenever the loop completes without `break`, regardless of iteration count.

## Alternative: Using label-break-value (RFC 2046)

Rust's [RFC 2046](https://rust-lang.github.io/rfcs/2046-label-break-value.html) provides labeled blocks with break values, which can achieve similar functionality:

**Using pythonic_for:**
```rust
use pythonic_for::pythonic_for;

let mut result = -1;
pythonic_for!(i in 0..5 {
    if i == 3 {
        result = i;
        break;
    }
} else {
    result = 100;
});
// result is 3
```

**Using label-break-value:**
```rust
let result = 'search: {
    for i in 0..5 {
        if i == 3 {
            break 'search i;
        }
    }
    100
};
// result is 3
```

**Tradeoffs:**
- **pythonic_for**: More familiar to Python developers, explicit `else` keyword makes intent clear, no extra indentation
- **label-break-value**: Native Rust feature (no macro), can return values from loops, requires extra indentation level

Both approaches are valid. Use `pythonic_for` if you prefer Python-style syntax or want to avoid extra indentation. Use label-break-value if you prefer native Rust features.

## Alternative Keyword: `final` instead of `else`

As discussed in [RFC #3361](https://github.com/rust-lang/rfcs/issues/3361), the `else` keyword can be confusing in loop contexts because it suggests a conditional relationship that doesn't exist. This crate supports `final` as an alternative keyword that may be clearer:

```rust
use pythonic_for::pythonic_for;

let mut result = -1;
pythonic_for!(i in 0..5 {
    if i == 3 {
        result = i;
        break;
    }
} final {
    result = 100;  // Only runs if loop completes without break
});
// result is 3 (final clause did not execute due to break)
```

The `final` keyword has identical semantics to `else` - it's purely a syntactic alternative. Use whichever keyword you find more readable. The `final` keyword emphasizes that this code runs at the "final" stage after loop completion, rather than suggesting a conditional branch.

## Step Value Behavior

When using `step` with ranges, the step value must not be zero. This matches Python's behavior:

```rust
use pythonic_for::pythonic_for;

// This will panic at runtime with a clear error message
pythonic_for!(i in 0..10, step = 0 {
    println!("{}", i);
});
// Panics: "pythonic_for: step argument must not be zero (matches Python's ValueError for range(step=0))"
```

**Python equivalent:**
```python
for i in range(0, 10, 0):  # Raises ValueError: range() arg 3 must not be zero
    print(i)
```

Negative step values are fully supported for reverse iteration:
```rust
pythonic_for!(i in 10..0, step = -2 {
    println!("{}", i);  // Prints: 10, 8, 6, 4, 2
});
```

## Design Rationale and Prior Art

This crate implements the for-else and while-else constructs as discussed in [Rust RFC #3361](https://github.com/rust-lang/rfcs/issues/3361). The semantics closely follow Python's implementation, which has proven useful for search patterns and conditional post-loop logic.

**Common Use Case - Search Pattern:**
```rust
use pythonic_for::pythonic_for;

let items = vec![1, 2, 3, 4, 5];
let target = 10;
let mut found_index = None;

pythonic_for!(i in 0..items.len() {
    if items[i] == target {
        found_index = Some(i);
        break;
    }
} else {
    println!("Item not found in collection");
});
```

**Prior Art:**
- **Python**: for-else and while-else since Python 2.0 (2000)
- **Ruby**: No direct equivalent, but similar patterns with `loop` and `break`
- **Swift**: No direct equivalent
- **Kotlin**: No direct equivalent
- **Other languages**: Some template languages (Jinja2, Twig) have for-else with different semantics (else runs only on zero iterations)

**Why a macro instead of language feature?**
This implementation uses procedural macros to provide the functionality without requiring changes to the Rust language itself. This allows developers to use Python-style loop semantics today while the Rust community discusses whether to add native language support via RFC #3361.

**Implementation approach:**
- Uses AST transformation to inject a `_break_occurred` flag
- Transforms `break` statements to set the flag before breaking
- Wraps loop in `catch_unwind` to detect panics
- Zero runtime overhead - all transformations happen at compile time

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pythonic-for = "0.1.0" # Replace with the latest version
```

## `pythonic_for!` Macro

### Basic Usage Without Else Clause

```rust
use pythonic_for::pythonic_for;

// Simple for loop without else clause
let mut sum = 0;
pythonic_for!(i in 0..5 {
    sum += i;
});
assert_eq!(sum, 10); // 0+1+2+3+4 = 10

// For loop with step value without else clause
let mut sum = 0;
pythonic_for!(i in 0..10, step = 2 {
    sum += i;
});
assert_eq!(sum, 20); // 0+2+4+6+8 = 20
```

### Usage With Else Clause

The `else` clause executes if the loop finishes normally (i.e., not via a `break` statement and no panics occurred).

```rust
use pythonic_for::pythonic_for;

// Basic for-else loop (else executes)
let mut found_val = -1;
pythonic_for!(i in 0..5 {
    if i == 10 { // Condition never met
        found_val = i;
        break;
    }
} else {
    // Loop completed without break
    found_val = 100;
});
assert_eq!(found_val, 100);

// For loop with break (else does not execute)
let mut found_val_break = -1;
pythonic_for!(i in 0..5 {
    if i == 3 {
        found_val_break = i;
        break;
    }
} else {
    // This will not execute
    found_val_break = 100;
});
assert_eq!(found_val_break, 3);

// Inclusive range
let mut sum = 0;
pythonic_for!(i in 1..=5 {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115

// Step value
let mut sum = 0;
pythonic_for!(i in 0..10, step = 2 {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 120); // 0+2+4+6+8+100 = 120

// Negative step
let mut sum = 0;
pythonic_for!(i in 10..0, step = -2 {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130

// Iterating over a collection
let vec = vec![1, 2, 3, 4, 5];
let mut sum = 0;
pythonic_for!(i in vec {
    sum += i;
} else {
    sum += 100;
});
assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
```

### Custom Iterators

The macro works with custom iterators as well:

```rust
# use pythonic_for::pythonic_for;
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

pythonic_for!(value in square_iter {
    sum += value;
} else {
    sum += 100;
});
assert_eq!(sum, 114); // 1+4+9+100 = 114
```

### Iterator Adapters

The `pythonic_for!` macro works seamlessly with all standard Iterator adapters:

```rust
# use pythonic_for::pythonic_for;
// Using enumerate
let letters = vec!['a', 'b', 'c'];
let mut s = String::new();
pythonic_for!(pair in letters.iter().enumerate() {
    let (idx, ch) = pair;
    s.push_str(&format!("({},{})", idx, ch));
});
assert_eq!(s, "(0,a)(1,b)(2,c)");

// Using take to limit iterations
let numbers = vec![1, 2, 3, 4, 5, 6];
let mut sum_take = 0;
pythonic_for!(n in numbers.iter().take(3) {
    sum_take += n;
});
assert_eq!(sum_take, 6); // 1+2+3

// Using skip to start from a specific position
let mut sum_skip = 0;
pythonic_for!(n in numbers.iter().skip(2) {
    sum_skip += n; // 3+4+5+6
});
assert_eq!(sum_skip, 18);


// Using flat_map for nested collections
let nested = vec![vec![1, 2], vec![3, 4]];
let mut sum_flat = 0;
pythonic_for!(n in nested.iter().flat_map(|v| v.iter()) {
    sum_flat += n;
});
assert_eq!(sum_flat, 10); // 1+2+3+4

// Using cycle with break for infinite iteration
let mut sum_cycle = 0;
let mut count = 0;
pythonic_for!(n in numbers.iter().cycle() {
    sum_cycle += n;
    count += 1;
    if count >= 10 { // Iterate 10 times over 1,2,3,4,5,6...
        break; 
    }
});
// (1+2+3+4+5+6) + (1+2+3+4) = 21 + 10 = 31
assert_eq!(sum_cycle, 31); 

// Other adapters like filter_map, chain, zip, etc. are also supported
```

## `pythonic_while!` Macro

The `pythonic_while!` macro provides Python-style `while` loops, also with an optional `else` clause that executes if the loop terminates normally (not via `break` and no panics).

### Basic `while` loop

```rust
# use pythonic_for::pythonic_while;
let mut count = 0;
let mut sum = 0;
pythonic_while!(count < 3; { // Condition
    // Body
    sum += count;
    count += 1;
});
assert_eq!(sum, 3); // 0 + 1 + 2
assert_eq!(count, 3);
```

### `while` loop with `else`

```rust
# use pythonic_for::pythonic_while;
let mut count = 0;
let mut sum = 0;
pythonic_while!(count < 3; { // Condition
    // Body
    sum += count;
    count += 1;
} else {
    // Else clause: executes because loop completed normally
    sum += 100;
});
assert_eq!(sum, 103); // 0 + 1 + 2 + 100
assert_eq!(count, 3);

// With break
let mut count_break = 0;
let mut sum_break = 0;
pythonic_while!(count_break < 5; {
    if count_break == 2 {
        break; // Loop terminates due to break
    }
    sum_break += count_break;
    count_break += 1;
} else {
    // Else clause: does NOT execute
    sum_break += 100;
});
assert_eq!(sum_break, 1); // 0 + 1
assert_eq!(count_break, 2);
```

### `do-while` loop variants

The `pythonic_while!` macro also supports a `do-while` style loop. The first block (do-body) always executes once. Then the condition is checked. If true, the second block (extra-body, or while-body) executes, and the loop continues.

```rust
# use pythonic_for::pythonic_while;
// do { body1 } while condition; { body2 }
let mut val = 0;
let mut iterations = 0;
pythonic_while!(do { // Do-body (always runs at least once)
    val += 1; 
} while val < 3; { // Condition (checked after do-body)
    // Extra-body (runs if condition is true)
    iterations += 1; 
    val += 1; // Modify condition variable here or in do-body
});
// Iteration 1: do{val=1}, cond(1<3 true), extra{iter=1, val=2}
// Iteration 2: do{val=3}, cond(3<3 false) -> loop ends
assert_eq!(val, 3);
assert_eq!(iterations, 1);


// do-while with else
let mut val_else = 0;
let mut iterations_else = 0;
let mut else_ran = false;
pythonic_while!(do {
    val_else += 1;
} while val_else < 3; {
    iterations_else += 1;
    val_else += 1;
    // Example break from extra_body
    // if val_else == 2 { break; } 
} else {
    else_ran = true;
});
assert_eq!(val_else, 3);
assert_eq!(iterations_else, 1);
assert!(else_ran); // Else runs as loop terminated normally by condition
```

## Control Flow

### Break Statements

The `pythonic_for!` and `pythonic_while!` macros automatically handle `break` statements. A `break` inside the loop body will prevent the `else` clause from executing.

```rust
# use pythonic_for::pythonic_for;
let mut sum = 0;
pythonic_for!(i in 0..5 {
    if i == 3 {
        break; // Exits loop, else clause is skipped
    }
    sum += i;
} else {
    sum = 99; // This will not execute
});
assert_eq!(sum, 3); // 0+1+2
```

### Nested Loops

-   **Native loop inside `pythonic_for!` / `pythonic_while!`**: A `break` inside a native inner loop (e.g., a standard Rust `for` or `while`) only breaks out of that inner native loop. The outer pythonic loop continues, and its `else` clause will execute if the pythonic loop itself completes normally.

    ```rust
    # use pythonic_for::pythonic_for;
    let mut outer_sum = 0;
    let mut inner_breaks = 0;
    pythonic_for!(i in 0..2 { // Outer pythonic loop
        outer_sum += i;
        for j in 0..5 { // Inner native loop
            if j == 1 {
                inner_breaks += 1;
                break; // This only breaks from the inner native loop
            }
        }
    } else {
        // This will still execute, as the pythonic_for loop completed normally.
        outer_sum += 100;
    });
    assert_eq!(inner_breaks, 2); // Inner loop broke twice
    assert_eq!(outer_sum, 101);  // 0 (i=0) + 1 (i=1) + 100 (else) = 101
    ```

-   **Nested `pythonic_for!` / `pythonic_while!` loops**: A `break` statement inside an inner *pythonic* loop will prevent its *own* `else` clause from executing. However, it does not automatically break the outer *pythonic* loop. The outer loop will continue its execution. If the outer loop completes normally (i.e., is not itself broken out of), its `else` clause will execute.

    ```rust
    # use pythonic_for::pythonic_for;
    let mut outer_sum = 0;
    let mut inner_sum = 0;
    let mut outer_else_ran = false;
    let mut inner_else_count = 0;

    pythonic_for!(i in 0..2 { // Outer pythonic loop
        outer_sum += i;
        pythonic_for!(j in 0..3 { // Inner pythonic loop
            inner_sum += j;
            if i == 0 && j == 1 {
                // Breaking only the inner pythonic loop
                break; 
            }
        } else {
            // Else for inner pythonic loop
            // Will run if inner loop is not broken.
            inner_else_count += 1; 
        });
        // Outer loop continues here
    } else {
        // Else for outer pythonic loop
        outer_else_ran = true;
        outer_sum += 100;
    });

    // Outer loop (i=0): outer_sum=0. 
    //   Inner loop (j=0, j=1, break): inner_sum = 0+1=1. Inner else does not run.
    // Outer loop (i=1): outer_sum=0+1=1.
    //   Inner loop (j=0, j=1, j=2, completes normally): inner_sum = 1 + (0+1+2) = 1+3=4. Inner else runs. inner_else_count=1.
    // Outer loop completes normally. Outer else runs: outer_else_ran=true, outer_sum = 1+100=101.
    
    assert_eq!(outer_sum, 101);
    assert_eq!(inner_sum, 4);
    assert_eq!(inner_else_count, 1); // Inner else ran once (for i=1)
    assert!(outer_else_ran);
    ```

## Error Handling

This crate provides consistent error handling mechanisms for both `pythonic_for!` and `pythonic_while!`.

1.  **Panic Handling**: If a panic occurs within the body of a `pythonic_for!` or `pythonic_while!` loop, the execution of the loop is immediately halted. The `else` clause associated with that loop will **not** be executed. The panic will propagate as usual unless caught by an outer `std::panic::catch_unwind`.

    ```rust
    # use pythonic_for::pythonic_for;
    # use std::panic;
    let mut else_executed = false;
    let result = panic::catch_unwind(|| {
        pythonic_for!(i in 0..5 {
            if i == 2 {
                panic!("Simulated error!");
            }
        } else {
            else_executed = true;
        });
    });

    assert!(result.is_err()); // The panic was caught
    assert!(!else_executed); // Else clause was skipped
    ```

2.  **Result-Based Error Handling**: For more controlled error management, you can use standard Rust `Result` types within the loop body. If an operation returns an `Err`, you can `break` from the loop. The `else` clause will not execute if the loop was terminated by a `break`.

    ```rust
    # use pythonic_for::pythonic_for;
    fn process_item(item: i32) -> Result<i32, String> {
        if item == 3 {
            Err(format!("Failed on item {}", item))
        } else {
            Ok(item * 2)
        }
    }

    let mut processed_sum = 0;
    let mut operation_status: Result<(), String> = Ok(());
    let mut else_ran = false;

    pythonic_for!(i in 0..5 {
        match process_item(i) {
            Ok(value) => {
                processed_sum += value;
            }
            Err(e) => {
                operation_status = Err(e);
                break; // Exit the loop on first error
            }
        }
    } else {
        // This only executes if all items were processed successfully (no break)
        else_ran = true;
    });

    assert!(else_ran || operation_status.is_err()); // Else runs OR an error occurred
    if operation_status.is_ok() {
        assert!(else_ran);
        assert_eq!(processed_sum, (0*2) + (1*2) + (2*2) + (4*2)); // 0+2+4+8 = 14
    } else {
        assert!(!else_ran);
        assert_eq!(processed_sum, (0*2) + (1*2) + (2*2)); // 0+2+4 = 6, then item 3 errors
        assert_eq!(operation_status, Err("Failed on item 3".to_string()));
    }
    ```

## Common Pitfalls and Best Practices

### Infinite Iterators with `pythonic_for!`

When using infinite iterators (e.g., `std::iter::repeat` or `Iterator::cycle()`) with `pythonic_for!`, the `else` clause is generally unreachable unless a `break` condition is included within the loop body.

```rust
# use pythonic_for::pythonic_for;
let data = [1,2,3];
let mut iteration_count = 0;
// LOGICAL ERROR if no break: The else clause might never execute with cycle()
// pythonic_for!(n in data.iter().cycle() {
//     // This will cycle through the collection indefinitely
// } else {
//     // This code is unreachable without a break in the loop body
// });

// CORRECT: Use a break condition with infinite iterators
pythonic_for!(n in data.iter().cycle() {
    iteration_count += 1;
    if iteration_count > 5 { // Ensure the loop terminates
        break; 
    }
} else {
    // This is reachable if the break condition above was NOT met (which it will be here).
    // If break is hit, this else is skipped.
});
assert_eq!(iteration_count, 6); // Loop runs 6 times (0..=5) then breaks.
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
