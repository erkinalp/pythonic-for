//! # Pythonic For
//!
//! A Rust crate that provides a Python-style `for` loop with an optional `else` clause.
//!
//! This crate allows users to iterate over a sequence and optionally execute an `else` clause
//! if no `break` is called and no error is returned during iteration.
//!
//! ## Examples
//!
//! ```no_run
//! use pythonic_for::pythonic_for;
//!
//! // Basic for-else loop - no break occurs, so else clause executes
//! let mut found = false;
//! pythonic_for!(i in 0..5 {
//!     if i == 10 { // This condition never matches in range 0..5
//!         found = true;
//!         break;
//!     }
//! } else {
//!     found = false; // This will execute
//! });
//! assert_eq!(found, false);
//!
//! // For loop without else clause
//! let mut sum = 0;
//! pythonic_for!(i in 0..5 {
//!     sum += i;
//! });
//! assert_eq!(sum, 10); // 0+1+2+3+4 = 10
//!
//! // For loop with break - break occurs, so else clause doesn't execute
//! let mut found = false;
//! pythonic_for!(i in 0..5 {
//!     if i == 3 { // This condition matches when i is 3
//!         found = true;
//!         break;
//!     }
//! } else {
//!     // This found = false should not be executed.
//! });
//! assert_eq!(found, true);
//!
//! // Inclusive range
//! let mut sum = 0;
//! pythonic_for!(i in 1..=5 {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
//!
//! // Step value
//! let mut sum = 0;
//! pythonic_for!(i in 0..10, step = 2 {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 120); // 0+2+4+6+8+100 = 120
//!
//! // Negative step
//! let mut sum = 0;
//! pythonic_for!(i in 10..0, step = -2 {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130
//!
//! // Iterating over a collection
//! let vec = vec![1, 2, 3, 4, 5];
//! let mut sum = 0;
//! pythonic_for!(i in vec {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
//! ```

/// # Parameters
///
/// * `var: ident` - The variable name to bind in each iteration
/// * `iterable: expr` - Any expression that implements `IntoIterator`
/// * `body: block` - The code block to execute for each iteration
/// * `else_body: block` - Optional code block to execute if no break occurs
/// * `range: expr` - A range expression (e.g., `0..5` or `1..=10`)
/// * `step: expr` - An integer expression for the step value (can be positive or negative)
///
/// # Return Value
///
/// This macro does not return a value; it executes the provided code blocks.
/// Macro that implements a Python-style for loop with an optional else clause.
///
/// This macro allows you to iterate over a sequence and optionally execute an else clause
/// if no break is called and no error is returned during iteration.
///
///
///
///
/// This macro does not return a value; it executes the provided code blocks.
///
/// # Syntax
///
/// The macro supports the following syntax patterns:
///
/// ```
/// use pythonic_for::pythonic_for;
///
/// // Basic for loop without else clause
/// let mut sum = 0;
/// pythonic_for!(i in 0..5 {
///     sum += i;
/// });
/// assert_eq!(sum, 10); // 0+1+2+3+4 = 10
///
/// // Basic for loop with else clause
/// let mut sum = 0;
/// pythonic_for!(i in 0..5 {
///     sum += i;
/// } else {
///     sum += 100;
/// });
/// assert_eq!(sum, 110); // 0+1+2+3+4+100 = 110
///
/// // For loop with range and step
/// let mut sum2 = 0;
/// pythonic_for!(i in 0..10, step = 2 {
///     sum2 += i;
/// } else {
///     sum2 += 100;
/// });
/// assert_eq!(sum2, 120); // 0+2+4+6+8+100 = 120
///
/// // For loop with inclusive range
/// let mut sum3 = 0;
/// pythonic_for!(i in 1..=5 {
///     sum3 += i;
/// } else {
///     sum3 += 100;
/// });
/// assert_eq!(sum3, 115); // 1+2+3+4+5+100 = 115
/// ```
///
/// # Features
///
/// - **Optional Else Clause**: The else clause is executed if the loop completes without a break
///   or an error, similar to Python's for-else construct. The else clause is optional.
/// - **Inclusive/Exclusive Ranges**: Supports both inclusive (`..=`) and exclusive (`..`) ranges.
/// - **Step Values**: Allows specifying a step value for iteration, including negative steps
///   for reverse iteration.
/// - **Error Handling**: Handles Rust errors similarly to how Python handles exceptions in for loops.
///   If an error occurs during iteration, the else clause is not executed. This is implemented
///   using `std::panic::catch_unwind` to catch any panics that might occur during iteration.
///
/// ```no_run
/// use pythonic_for::pythonic_for;
/// use std::panic;
///
/// // Example with error handling
/// let mut result = 0;
///
/// // This will panic during iteration when i == 3
/// let result_with_error = panic::catch_unwind(panic::AssertUnwindSafe(|| {
///     let mut inner_result = 0;
///     pythonic_for!(i in 0..5 {
///         if i == 3 {
///             // Simulate an error
///             panic!("Error during iteration");
///         }
///         inner_result += i;
///     } else {
///         inner_result = 100; // This should not execute
///     });
///     // Update the outer result
///     result = inner_result;
/// }));
///
/// // The else clause is not executed because an error occurred
/// assert!(result_with_error.is_err());
/// // result remains 0 because the panic occurred (or whatever value it had before the panic in pythonic_for scope)
/// // Assuming inner_result is not assigned back to result if panic happens.
/// // The exact value of `result` depends on how `pythonic_for` handles assignments before panic.
/// // With `catch_unwind`, `result` would retain its value from before `pythonic_for` if `inner_result` isn't assigned.
/// // If `pythonic_for` is a block expression, `result` would be assigned the outcome of that block.
/// // The current `pythonic_for` doesn't return a value, it's a statement.
/// // So `result` would be unchanged (0) if panic occurs before `result = inner_result;`
/// assert_eq!(result, 0); 
/// ```
// Re-export the procedural macro
pub use pythonic_for_proc_macros::pythonic_for;

#[cfg(test)]
mod tests {
    use super::*; 
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

    #[test]
    fn test_basic_for_else() {
        let mut found = false;
        pythonic_for!(i in 0..5 {
            if i == 10 { 
                found = true;
                break;
            }
        } else {
            found = false; 
        });
        assert_eq!(found, false);
    }

    #[test]
    fn test_for_without_else() {
        let mut sum = 0;
        pythonic_for!(i in 0..5 {
            sum += i;
        });
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_for_with_break() {
        let mut found = false;
        pythonic_for!(i in 0..5 {
            if i == 3 { 
                found = true;
                break;
            }
        } else {
            // This part should not execute
        });
        assert_eq!(found, true); 
    }
    
    #[test]
    fn test_inclusive_range() {
        let mut sum = 0;
        pythonic_for!(i in 1..=5 {
            sum += i;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 115);
    }

    #[test]
    fn test_step_value() {
        let mut sum = 0;
        pythonic_for!(i in 0..10, step = 2 {
            sum += i;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 120); 
    }

    #[test]
    fn test_step_value_without_else() {
        let mut sum = 0;
        pythonic_for!(i in 0..10, step = 2 {
            sum += i;
        });
        assert_eq!(sum, 20); 
    }

    #[test]
    fn test_negative_step() {
        let mut sum = 0;
        pythonic_for!(i in 10..0, step = -2 {
            sum += i;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 130); 
    }

    #[test]
    fn test_negative_step_without_else() {
        let mut sum = 0;
        pythonic_for!(i in 10..0, step = -2 {
            sum += i;
        });
        assert_eq!(sum, 30); 
    }

    #[test]
    fn test_iterable() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        pythonic_for!(i in vec { 
            sum += i;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 115); 
    }

    #[test]
    fn test_iterable_without_else() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        pythonic_for!(i in vec.iter() { 
            sum += i;
        });
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_empty_range_else_executes() {
        let mut else_ran = false;
        pythonic_for!(i in 0..0 {
            panic!("Should not iterate on empty range");
        } else {
            else_ran = true;
        });
        assert!(else_ran, "Else should execute when loop never iterates (empty range)");
    }

    #[test]
    fn test_empty_iterator_else_executes() {
        let empty_vec: Vec<i32> = vec![];
        let mut else_ran = false;
        pythonic_for!(i in empty_vec {
            panic!("Should not iterate on empty iterator");
        } else {
            else_ran = true;
        });
        assert!(else_ran, "Else should execute when iterator is empty");
    }

    #[test]
    #[should_panic(expected = "step argument must not be zero")]
    fn test_step_zero_panics() {
        pythonic_for!(i in 0..10, step = 0 {
            // Should panic before any iteration
        });
    }

    #[test]
    fn test_final_keyword_works() {
        let mut sum = 0;
        pythonic_for!(i in 0..5 {
            sum += i;
        } final {
            sum += 100;
        });
        assert_eq!(sum, 110);
    }

    #[test]
    fn test_final_keyword_with_break() {
        let mut sum = 0;
        pythonic_for!(i in 0..5 {
            sum += i;
            if i == 2 {
                break;
            }
        } final {
            sum += 100;
        });
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_final_keyword_empty_range() {
        let mut final_ran = false;
        pythonic_for!(i in 5..5 {
            panic!("Should not iterate");
        } final {
            final_ran = true;
        });
        assert!(final_ran, "Final should execute on empty range");
    }

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);

        let mut sum = 0;
        pythonic_for!(entry in map {
            sum += entry.1;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 106);
    }

    #[test]
    fn test_hashset() {
        let mut set = HashSet::new();
        set.insert(5);
        set.insert(10);
        set.insert(15);

        let mut sum = 0;
        pythonic_for!(value in set {
            sum += value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 130);
    }

    #[test]
    fn test_vecdeque() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        let mut sum = 0;
        pythonic_for!(value in deque {
            sum += value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 106);
    }

    #[test]
    fn test_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let mut sum = 0;
        pythonic_for!(entry in map {
            sum += entry.1;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 106);
    }

    #[test]
    fn test_btreeset() {
        let mut set = BTreeSet::new();
        set.insert(5);
        set.insert(10);
        set.insert(15);

        let mut sum = 0;
        pythonic_for!(value in set {
            sum += value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 130);
    }

    #[test]
    fn test_custom_iterator() {
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
        assert_eq!(sum, 114);
    }

    #[test]
    fn test_filter_map() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        let even_doubles = numbers.iter().filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None });
        pythonic_for!(value in even_doubles {
            sum += value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 112);
    }

    #[test]
    fn test_chain_iterator() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];
        let mut sum = 0;
        let chained = first.iter().chain(second.iter());
        pythonic_for!(value in chained {
            sum += *value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 121);
    }

    #[test]
    fn test_zip_iterator() {
        let numbers = vec![1, 2, 3];
        let letters = vec!['a', 'b', 'c'];
        let mut result = String::new();
        let zipped = numbers.iter().zip(letters.iter());
        pythonic_for!(pair in zipped {
            result.push_str(&format!("{}{}", pair.0, pair.1));
        } else {
            result.push_str("done");
        });
        assert_eq!(result, "1a2b3cdone");
    }

    #[test]
    fn test_enumerate_iterator() {
        let letters = vec!['a', 'b', 'c'];
        let mut result = String::new();
        let enumerated = letters.iter().enumerate();
        pythonic_for!(pair in enumerated {
            result.push_str(&format!("{}{}", pair.0, pair.1));
        } else {
            result.push_str("done");
        });
        assert_eq!(result, "0a1b2cdone");
    }

    #[test]
    fn test_take_iterator() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        let taken = numbers.iter().take(3);
        pythonic_for!(value in taken {
            sum += *value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 106);
    }

    #[test]
    fn test_skip_iterator() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        let skipped = numbers.iter().skip(2);
        pythonic_for!(value in skipped {
            sum += *value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 112);
    }

    #[test]
    fn test_flat_map_iterator() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        let mut sum = 0;
        let flattened = nested.iter().flat_map(|v| v.iter());
        pythonic_for!(value in flattened {
            sum += *value;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 110);
    }

    #[test]
    fn test_nested_loops_without_else() {
        let mut sum = 0;
        let mut outer_count = 0;
        let mut inner_breaks = 0;
        pythonic_for!(i in 0..3 {
            outer_count += 1;
            for j in 0..3 { // Native for loop
                sum += i * j;
                if j == 1 {
                    inner_breaks += 1;
                    break; // Breaks native for loop
                }
            }
            if i == 1 { // This break is for pythonic_for!
                break;
            }
        });
        assert_eq!(outer_count, 2); // i=0, i=1
        assert_eq!(inner_breaks, 2); // j loop breaks for i=0 and i=1
        assert_eq!(sum, 1); // i=0: sum=0*0=0; i=1: sum=0 + 1*0 + 1*1=1
    }
    
    #[test]
    fn test_nested_loops_with_else() {
        let mut sum = 0;
        let mut outer_count = 0;
        let mut inner_breaks = 0;
        let mut else_executed = false;
        pythonic_for!(i in 0..3 {
            outer_count += 1;
            for j in 0..3 { // Native for loop
                sum += i * j;
                if j == 1 {
                    inner_breaks += 1;
                    break; // Breaks native for loop
                }
            }
            if i == 1 { // This break is for pythonic_for!
                break; 
            }
        } else {
            else_executed = true;
            sum += 100;
        });
        assert_eq!(outer_count, 2); // i=0, i=1
        assert_eq!(inner_breaks, 2); // j loop breaks for i=0 and i=1
        assert_eq!(sum, 1); // Outer else should NOT execute due to `break` when i = 1
        assert_eq!(else_executed, false); // Outer else should NOT execute
    }

    #[test]
    fn test_nested_pythonic_for_inner_break() {
        let mut outer_sum = 0;
        let mut inner_sum = 0;
        let mut outer_else_executed = false;
        let mut inner_else_executed = false;
        pythonic_for!(i in 0..3 { // Outer pythonic_for
            outer_sum += i;
            pythonic_for!(j in 0..3 { // Inner pythonic_for
                inner_sum += j;
                if j == 1 {
                    break; // Breaks inner pythonic_for
                }
            } else {
                inner_else_executed = true; // This should NOT execute
            });
        } else { // Else for outer pythonic_for
            outer_else_executed = true; // This SHOULD execute as outer loop completes
            outer_sum += 100;
        });
        assert_eq!(outer_sum, 103); // 0+1+2+100
        assert_eq!(inner_sum, 3);   // For i=0: (j=0, j=1, break) -> inner_sum=1. 
                                    // For i=1: (j=0, j=1, break) -> inner_sum=1+1=2.
                                    // For i=2: (j=0, j=1, break) -> inner_sum=2+1=3.
        assert_eq!(inner_else_executed, false); // Inner `else` does not execute because of inner `break`.
        assert_eq!(outer_else_executed, true); // Outer `else` executes.
    }

    #[test]
    fn test_nested_pythonic_for_outer_break() {
        let mut outer_sum = 0;
        let mut inner_sum = 0;
        let mut outer_else_executed = false;
        let mut inner_else_count = 0;
        pythonic_for!(i in 0..3 { // Outer pythonic_for
            outer_sum += i;
            pythonic_for!(j in 0..3 { // Inner pythonic_for
                inner_sum += j;
            } else { // Else for inner pythonic_for
                inner_else_count += 1; // This should execute for i=0 and i=1
                inner_sum += 10; 
            });
            if i == 1 { // This break is for outer pythonic_for
                break;
            }
        } else { // Else for outer pythonic_for
            outer_else_executed = true; // This should NOT execute
        });
        assert_eq!(outer_sum, 1);   // i=0 (sum=0), i=1 (sum=0+1=1), then outer break.
        // For i=0: inner loop (0+1+2=3), inner_else (sum=3+10=13). inner_sum_total = 13.
        // For i=1: inner loop (0+1+2=3), inner_else (sum=3+10=13). inner_sum_total = 13+13=26.
        assert_eq!(inner_sum, 26);  
        assert_eq!(inner_else_count, 2); // Inner `else` runs for i=0 and i=1.
        assert_eq!(outer_else_executed, false); // Outer `else` does not run due to outer `break`.
    }

    // --- Tests for pythonic_while! ---

    #[test]
    fn test_while_basic_no_else() {
        let mut count = 0;
        let mut sum = 0;
        pythonic_while!(count < 3; {
            sum += count;
            count += 1;
        });
        assert_eq!(sum, 3); // 0 + 1 + 2
        assert_eq!(count, 3);
    }

    #[test]
    fn test_while_basic_with_else_runs() {
        let mut count = 0;
        let mut sum = 0;
        pythonic_while!(count < 3; {
            sum += count;
            count += 1;
        } else {
            sum += 100;
        });
        assert_eq!(sum, 103); // 0 + 1 + 2 + 100
        assert_eq!(count, 3);
    }

    #[test]
    fn test_while_basic_with_else_break() {
        let mut count = 0;
        let mut sum = 0;
        pythonic_while!(count < 5; {
            if count == 2 {
                break;
            }
            sum += count;
            count += 1;
        } else {
            sum += 100; // Should not run
        });
        assert_eq!(sum, 1); // 0 + 1
        assert_eq!(count, 2);
    }

    #[test]
    fn test_do_while_no_else() {
        let mut count = 0;
        let mut sum = 0;
        // Simulates: do { sum += count; count += 1; } while count < 3; (extra body is empty)
        pythonic_while!(do {
            sum += count;
            count += 1;
        }; while count < 3; {}); // extra_body is empty
        assert_eq!(sum, 3); // 0 (count=1) + 1 (count=2) + 2 (count=3) -> loop exits as count is not < 3
        assert_eq!(count, 3);
    }
    
    #[test]
    fn test_do_while_no_else_with_extra_body() {
        let mut count = 0;
        let mut sum = 0;
        pythonic_while!(do { // Body 1
            sum += count;    // iter 1: sum=0, count=0 -> sum=0
                             // iter 2: sum=1, count=1 -> sum=1+1=2
        }; while count < 2; { // Condition checked: count=0<2 (true); count=1<2 (true); count=2<2 (false)
            count += 1;      // Extra_body (Body 2)
                             // iter 1: count=0 -> count=1
                             // iter 2: count=1 -> count=2
        });
        // Iteration 1: body1 (sum=0, count=0), cond (0<2 true), extra_body (count=1)
        // Iteration 2: body1 (sum=0+1=1, count=1), cond (1<2 true), extra_body (count=2)
        // Iteration 3: body1 (sum=1+2=3, count=2), cond (2<2 false), loop terminates.
        assert_eq!(sum, 3); 
        assert_eq!(count, 2); // count becomes 2 in the last extra_body, then body1 runs once more.
                               // Let's re-trace:
                               // 1. count=0, sum=0
                               // 2. body: sum=0 (0+0), count=0
                               // 3. cond: 0 < 2 (true)
                               // 4. extra_body: count=1
                               // 5. body: sum=1 (0+1), count=1
                               // 6. cond: 1 < 2 (true)
                               // 7. extra_body: count=2
                               // 8. body: sum=3 (1+2), count=2
                               // 9. cond: 2 < 2 (false) -> terminate
        assert_eq!(sum, 3);
        assert_eq!(count, 2);
    }


    #[test]
    fn test_do_while_with_else_runs() {
        let mut count = 0;
        let mut sum = 0;
        pythonic_while!(do {
            sum += count; // iter1: sum=0,c=0; iter2: sum=1,c=1; iter3: sum=3,c=2
        }; while count < 2; { // cond1: 0<2 T; cond2: 1<2 T; cond3: 2<2 F
            count += 1;   // iter1: c=1; iter2: c=2
        } else {
            sum += 100;
        });
        assert_eq!(sum, 103); // 3 + 100
        assert_eq!(count, 2);
    }

    #[test]
    fn test_do_while_with_else_break_in_first_body() {
        let mut count = 0;
        let mut sum = 0;
        pythonic_while!(do {
            if count == 1 {
                break;
            }
            sum += count; // iter1: sum=0, c=0
            count += 1;   // iter1: c=1
        }; while count < 3; {
            // extra_body, should not be reached if break occurs in first body before this
        } else {
            sum += 100; // Should not run
        });
        // Iteration 1: count=0. body: sum=0, count=1.
        // Iteration 2: count=1. body: break.
        assert_eq!(sum, 0); 
        assert_eq!(count, 1);
    }

    #[test]
    fn test_do_while_with_else_break_in_extra_body() {
        let mut count = 0;
        let mut sum = 0;
        pythonic_while!(do {
            sum += count;    // iter1: sum=0, c=0; iter2: sum=1, c=1
        }; while count < 3; { // cond1: 0<3 T; cond2: 1<3 T
            count += 1;      // iter1: c=1; iter2: c=2
            if count == 2 {
                break;       // Breaks here on second iteration of extra_body
            }
        } else {
            sum += 100; // Should not run
        });
        // Iteration 1: body (sum=0, c=0), cond (0<3 T), extra_body (c=1)
        // Iteration 2: body (sum=0+1=1, c=1), cond (1<3 T), extra_body (c=2, break)
        assert_eq!(sum, 1); 
        assert_eq!(count, 2);
    }

    #[test]
    fn test_while_false_condition_else_runs() {
        let mut body_executed = false;
        let mut else_executed = false;
        pythonic_while!(false; {
            body_executed = true;
        } else {
            else_executed = true;
        });
        assert!(!body_executed);
        assert!(else_executed);
    }
    
    #[test]
    fn test_do_while_false_condition_else_runs() {
        // For do-while, the first body always runs once.
        // Then condition is checked. If false, loop terminates, else runs.
        let mut body_executed_count = 0;
        let mut extra_body_executed = false;
        let mut else_executed = false;

        pythonic_while!(do {
            body_executed_count += 1; // Runs once
        }; while false; { // Condition is immediately false
            extra_body_executed = true; // Should not run
        } else {
            else_executed = true; // Should run
        });
        assert_eq!(body_executed_count, 1);
        assert!(!extra_body_executed);
        assert!(else_executed);
    }


    #[test]
    fn test_while_panic_handling_no_else() {
        let result = std::panic::catch_unwind(|| {
            let mut count = 0;
            pythonic_while!(count < 3; {
                if count == 1 {
                    panic!("Test panic in while no else");
                }
                count += 1;
            });
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_while_panic_handling_with_else() {
        let mut else_ran = false;
        let result = std::panic::catch_unwind(|| {
            let mut count = 0;
            pythonic_while!(count < 3; {
                if count == 1 {
                    panic!("Test panic in while with else");
                }
                count += 1;
            } else {
                else_ran = true;
            });
        });
        assert!(result.is_err());
        assert!(!else_ran, "Else clause should not run if panic occurred in body");
    }
    
    #[test]
    fn test_do_while_panic_in_first_body_no_else() {
        let result = std::panic::catch_unwind(|| {
            let mut count = 0;
            pythonic_while!(do {
                if count == 0 { // Panic on first execution of body
                    panic!("Test panic in do-while first body no else");
                }
                count += 1;
            }; while count < 2; {});
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_do_while_panic_in_first_body_with_else() {
        let mut else_ran = false;
        let result = std::panic::catch_unwind(|| {
            let mut count = 0;
            pythonic_while!(do {
                if count == 0 {
                     panic!("Test panic in do-while first body with else");
                }
                count += 1;
            }; while count < 2; {}
            else {
                else_ran = true;
            });
        });
        assert!(result.is_err());
        assert!(!else_ran);
    }
    
    #[test]
    fn test_do_while_panic_in_extra_body_no_else() {
        let result = std::panic::catch_unwind(|| {
            let mut count = 0;
            pythonic_while!(do {
                // This part runs fine
                count = 5; 
            }; while count < 10; { // Condition is true
                panic!("Test panic in do-while extra_body no else");
            });
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_do_while_panic_in_extra_body_with_else() {
        let mut else_ran = false;
        let result = std::panic::catch_unwind(|| {
            let mut count = 0;
             pythonic_while!(do {
                count = 5;
            }; while count < 10; { // Condition is true
                panic!("Test panic in do-while extra_body with else");
            } else {
                else_ran = true;
            });
        });
        assert!(result.is_err());
        assert!(!else_ran);
    }

}

#[doc(hidden)]
#[inline]
pub fn is_cycle_iterator<I>(_iter: &I) -> bool { // Renamed from _is_likely_cycle
    let type_name = std::any::type_name::<I>();
    type_name.contains("std::iter::Cycle") || type_name.contains("::Cycle<")
}

#[deprecated(
    since = "0.1.0",
    note = "Warning: Using cycle() iterator with else clause may create a logical error. \
           The else clause will never execute because cycle() creates an infinite iterator. \
           Consider adding a break condition in your loop body if this is intentional."
)] // Note does not mention the helper function name, so it's fine.
#[doc(hidden)]
#[inline]
pub fn _warn_cycle_with_else() {
}

/// immediately skip to the else clause without any iteration.
#[doc(hidden)]
#[inline]
pub fn is_false_condition(condition: &str) -> bool { // Renamed from _is_likely_false_condition
    condition == "false" || 
    condition == "0 == 1" || 
    condition == "1 == 0" ||
    condition == "0 != 0" ||
    condition == "1 != 1" ||
    (condition.contains(" < ") && condition.split(" < ").collect::<Vec<&str>>().len() == 2 &&
        condition.split(" < ").next().unwrap_or_default().trim().parse::<i32>().is_ok() &&
        condition.split(" < ").nth(1).unwrap_or_default().trim().parse::<i32>().is_ok() &&
        condition.split(" < ").next().unwrap_or_default().trim().parse::<i32>().unwrap_or(1) >= 
        condition.split(" < ").nth(1).unwrap_or_default().trim().parse::<i32>().unwrap_or(0))
}

#[deprecated(
    since = "0.1.0",
    note = "Warning: Using a while loop with a condition known to be false at compile time with an else clause may create a logical error. \
           The loop body will never execute and control will immediately pass to the else clause. \
           Consider using an if-else statement instead if this is intentional."
)]
#[doc(hidden)]
#[inline]
pub fn _warn_false_condition_with_else() {
}

/// A macro that provides Python-style while loops with an optional else clause.
///
/// This macro allows you to create while loops that optionally execute an else clause
/// if no break is called and no error is returned during iteration.
///
/// # Examples
///
/// ```
/// use pythonic_for::pythonic_while; 
///
/// // Basic while-else loop - no break occurs, so else clause executes
/// let mut counter = 0;
/// let mut result = 0;
///
/// pythonic_while!(counter < 5; {
///     result += counter;
///     counter += 1;
/// } else {
///     result += 100;
/// });
///
/// assert_eq!(result, 110); 
/// ```
///
/// ```
/// use pythonic_for::pythonic_while; 
///
/// let mut counter = 0;
/// let mut result = 0;
///
/// pythonic_while!(counter < 1; {
///     result += counter;
///     counter += 1;
/// });
///
/// assert_eq!(result, 0); 
/// assert_eq!(counter, 1); 
/// ```
#[macro_export]
macro_rules! pythonic_while {
    // Standard while loop with a body but no else clause
    ($condition:expr; $body:block) => {
        {
            let mut _break_occurred = false;
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: while $condition {
                    $crate::_internal_pythonic_while_body!('pythonic_while_loop, { $body }); 
                    if _break_occurred { 
                        break 'pythonic_while_loop;
                    }
                }
            }));
            // If __result.is_err(), panic is caught.
        }
    };

    // Standard while loop with a body and an else clause
    ($condition:expr; $body:block else $else_body:block) => {
        {
            let condition_str = stringify!($condition);
            let _false_condition_detected = $crate::is_false_condition(condition_str); // Updated call site
            
            if _false_condition_detected {
                $crate::_warn_false_condition_with_else();
            }
            
            let mut _break_occurred = false;
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: while $condition {
                    $crate::_internal_pythonic_while_body!('pythonic_while_loop, { $body });
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                }
            }));

            if !_break_occurred && __result.is_ok() {
                $else_body
            }
        }
    };

    // Do-while pattern without an else clause
    (do $body:block; while $condition:expr; $extra_body:block) => {
        {
            let mut _break_occurred = false;
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: loop {
                    $crate::_internal_pythonic_while_body!('pythonic_while_loop, { $body });
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                    if !($condition) {
                        break 'pythonic_while_loop; 
                    }
                    $crate::_internal_pythonic_while_body!('pythonic_while_loop, { $extra_body }); 
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                }
            }));
            // If __result.is_err(), panic is caught.
        }
    };

    // Do-while pattern with an else clause
    (do $body:block; while $condition:expr; $extra_body:block else $else_body:block) => {
        {
            let condition_str = stringify!($condition);
            let _false_condition_detected = $crate::is_false_condition(condition_str); // Updated call site
            
            if _false_condition_detected {
                $crate::_warn_false_condition_with_else();
            }
            
            let mut _break_occurred = false;
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: loop {
                    $crate::_internal_pythonic_while_body!('pythonic_while_loop, { $body });
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                    if !($condition) {
                        break 'pythonic_while_loop;
                    }
                    $crate::_internal_pythonic_while_body!('pythonic_while_loop, { $extra_body });
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                }
            }));

            if !_break_occurred && __result.is_ok() {
                $else_body
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _internal_pythonic_while_body {
    ($label:lifetime, $body:block) => {
        // Pass the label and the body to the transform_body proc macro
        pythonic_for_proc_macros::transform_body!($label, { $body })
    };
}
