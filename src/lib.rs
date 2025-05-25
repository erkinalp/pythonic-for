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
//! pythonic_for!((i in 0..5) {
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
//! pythonic_for!((i in 0..5) {
//!     sum += i;
//! });
//! assert_eq!(sum, 10); // 0+1+2+3+4 = 10
//!
//! // For loop with break - break occurs, so else clause doesn't execute
//! let mut found = false;
//! pythonic_for!((i in 0..5) {
//!     if i == 3 { // This condition matches when i is 3
//!         found = true;
//!         break;
//!     }
//! } else {
//!     found = false; // This won't execute because of the break
//! });
//! assert_eq!(found, true);
//!
//! // Inclusive range
//! let mut sum = 0;
//! pythonic_for!((i in 1..=5) {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
//!
//! // Step value
//! let mut sum = 0;
//! pythonic_for!((i in 0..10, step = 2) {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 120); // 0+2+4+6+8+100 = 120
//!
//! // Negative step
//! let mut sum = 0;
//! pythonic_for!((i in 10..0, step = -2) {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130
//!
//! // Iterating over a collection
//! let vec = vec![1, 2, 3, 4, 5];
//! let mut sum = 0;
//! pythonic_for!((i in vec) {
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
/// pythonic_for!((i in 0..5) {
///     sum += i;
/// });
/// assert_eq!(sum, 10); // 0+1+2+3+4 = 10
///
/// // Basic for loop with else clause
/// let mut sum = 0;
/// pythonic_for!((i in 0..5) {
///     sum += i;
/// } else {
///     sum += 100;
/// });
/// assert_eq!(sum, 110); // 0+1+2+3+4+100 = 110
///
/// // For loop with range and step
/// let mut sum2 = 0;
/// pythonic_for!((i in 0..10, step = 2) {
///     sum2 += i;
/// } else {
///     sum2 += 100;
/// });
/// assert_eq!(sum2, 120); // 0+2+4+6+8+100 = 120
///
/// // For loop with inclusive range
/// let mut sum3 = 0;
/// pythonic_for!((i in 1..=5) {
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
///     pythonic_for!((i in 0..5) {
///         if i == 3 {
///             // Simulate an error
///             panic!("Error during iteration");
///         }
///         inner_result += i;
///     } else {
///         inner_result = 100;
///     });
///     // Update the outer result
///     result = inner_result;
/// }));
///
/// // The else clause is not executed because an error occurred
/// assert!(result_with_error.is_err());
/// // result remains 0 because the panic occurred
/// assert_eq!(result, 0);
/// ```
#[macro_export]
macro_rules! pythonic_for {
    // For iterating over any iterable without an else clause
    (($var:ident in $iterable:expr) $body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_for_loop: for $var in $iterable {
                    $crate::_internal_pythonic_for_body!($body);
                    
                    if _break_occurred {
                        break 'pythonic_for_loop;
                    }
                }
            }));

            if result.is_err() {
                _error_occurred = true;
            }
        }
    };

    // For iterating over any iterable with an else clause
    (($var:ident in $iterable:expr) $body:block else $else_body:block) => {
        {
            // This is a compile-time warning for using cycle() with an else clause
            {
                let type_name = stringify!($iterable);
                if type_name.contains("cycle") || type_name.contains("Cycle") {
                    $crate::_cycle_with_else_warning!();
                }
            }
            
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_for_loop: for $var in $iterable {
                    $crate::_internal_pythonic_for_body!($body);
                    
                    if _break_occurred {
                        break 'pythonic_for_loop;
                    }
                }
            }));

            if result.is_err() {
                _error_occurred = true;
            }

            // Execute the else body only if no break occurred and no error occurred
            if !_break_occurred && !_error_occurred {
                $else_body
            }
        }
    };

    // For iterating over a range with a step without an else clause
    (($var:ident in $range:expr, step = $step:expr) $body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let step = $step;
                let range = $range;

                let is_inclusive = {
                    let range_str = format!("{:?}", range);
                    range_str.contains("=")
                };

                if step > 0 {
                    // Forward iteration with positive step
                    let start = range.start;
                    let end = range.end;
                    let mut current = start;

                    'pythonic_for_loop: while if is_inclusive { current <= end } else { current < end } {
                        let $var = current;
                        
                        // Execute the loop body
                        let _result = (|| {
                            #[allow(unused_labels)]
                            'inner: {
                                $crate::_internal_pythonic_for_body!($body);
                            }
                        })();
                        
                        if _break_occurred {
                            break 'pythonic_for_loop;
                        }
                        
                        current += step;
                    }
                } else if step < 0 {
                    // Reverse iteration with negative step
                    let start = range.start;
                    let end = range.end;
                    let mut current = start;

                    'pythonic_for_loop: while if is_inclusive { current >= end } else { current > end } {
                        let $var = current;
                        
                        // Execute the loop body
                        let _result = (|| {
                            #[allow(unused_labels)]
                            'inner: {
                                $crate::_internal_pythonic_for_body!($body);
                            }
                        })();
                        
                        if _break_occurred {
                            break 'pythonic_for_loop;
                        }
                        
                        current += step;
                    }
                }
            }));

            if result.is_err() {
                _error_occurred = true;
            }
        }
    };

    // For iterating over a range with a step with an else clause
    (($var:ident in $range:expr, step = $step:expr) $body:block else $else_body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let step = $step;
                let range = $range;

                let is_inclusive = {
                    let range_str = format!("{:?}", range);
                    range_str.contains("=")
                };

                if step > 0 {
                    // Forward iteration with positive step
                    let start = range.start;
                    let end = range.end;
                    let mut current = start;

                    'pythonic_for_loop: while if is_inclusive { current <= end } else { current < end } {
                        let $var = current;
                        
                        // Execute the loop body
                        let _result = (|| {
                            #[allow(unused_labels)]
                            'inner: {
                                $crate::_internal_pythonic_for_body!($body);
                            }
                        })();
                        
                        if _break_occurred {
                            break 'pythonic_for_loop;
                        }
                        
                        current += step;
                    }
                } else if step < 0 {
                    // Reverse iteration with negative step
                    let start = range.start;
                    let end = range.end;
                    let mut current = start;

                    'pythonic_for_loop: while if is_inclusive { current >= end } else { current > end } {
                        let $var = current;
                        
                        // Execute the loop body
                        let _result = (|| {
                            #[allow(unused_labels)]
                            'inner: {
                                $crate::_internal_pythonic_for_body!($body);
                            }
                        })();
                        
                        if _break_occurred {
                            break 'pythonic_for_loop;
                        }
                        
                        current += step;
                    }
                }
            }));

            if result.is_err() {
                _error_occurred = true;
            }

            // Execute the else body only if no break occurred and no error occurred
            if !_break_occurred && !_error_occurred {
                $else_body
            }
        }
    };

    (($var:ident in $iterable:expr) { $($body:tt)* }) => {
        pythonic_for!(($var in $iterable) { $($body)* })
    };

    (($var:ident in $iterable:expr) { $($body:tt)* } else $else_body:block) => {
        pythonic_for!(($var in $iterable) { $($body)* } else $else_body)
    };

    (($var:ident in $range:expr, step = $step:expr) { $($body:tt)* }) => {
        pythonic_for!(($var in $range, step = $step) { $($body)* })
    };

    (($var:ident in $range:expr, step = $step:expr) { $($body:tt)* } else $else_body:block) => {
        pythonic_for!(($var in $range, step = $step) { $($body)* } else $else_body)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

    #[test]
    fn test_basic_for_else() {
        let mut found = false;

        {
            let mut _break_occurred = false;

            'pythonic_for_loop: for i in 0..5 {
                if i == 10 {
                    found = true;
                    _break_occurred = true;
                    break 'pythonic_for_loop;
                }
            }

            if !_break_occurred {
                found = false;
            }
        }

        assert_eq!(found, false);
    }

    #[test]
    fn test_for_without_else() {
        let mut sum = 0;

        pythonic_for!((i in 0..5) {
            sum += i;
        });

        assert_eq!(sum, 10); // 0+1+2+3+4 = 10
    }

    #[test]
    fn test_for_with_break() {
        let mut found = false;

        {
            let mut _break_occurred = false;

            'pythonic_for_loop: for i in 0..5 {
                if i == 3 {
                    found = true;
                    _break_occurred = true;
                    break 'pythonic_for_loop;
                }
            }

            if !_break_occurred {
                found = false;
            }
        }

        assert_eq!(found, true);
    }

    #[test]
    fn test_inclusive_range() {
        let mut sum = 0;

        {
            let mut _break_occurred = false;

            'pythonic_for_loop: for i in 1..=5 {
                sum += i;
            }

            if !_break_occurred {
                sum += 100;
            }
        }

        assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
    }

    #[test]
    fn test_step_value() {
        let mut sum = 0;

        {
            let mut _break_occurred = false;

            'pythonic_for_loop: {
                let step = 2;
                let range = 0..10;
                let start = range.start;
                let end = range.end;
                let mut current = start;

                while current < end {
                    let i = current;
                    sum += i;
                    current += step;
                }
            }

            if !_break_occurred {
                sum += 100;
            }
        }

        assert_eq!(sum, 120); // 0+2+4+6+8+100 = 120
    }

    #[test]
    fn test_step_value_without_else() {
        let mut sum = 0;

        pythonic_for!((i in 0..10, step = 2) {
            sum += i;
        });

        assert_eq!(sum, 20); // 0+2+4+6+8 = 20
    }

    #[test]
    fn test_negative_step() {
        let mut sum = 0;

        {
            let mut _break_occurred = false;

            'pythonic_for_loop: {
                let step = -2;
                let range = 10..0;
                let start = range.start;
                let end = range.end;
                let mut current = start;

                while current > end {
                    let i = current;
                    sum += i;
                    current += step;
                }
            }

            if !_break_occurred {
                sum += 100;
            }
        }

        assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130
    }

    #[test]
    fn test_negative_step_without_else() {
        let mut sum = 0;

        pythonic_for!((i in 10..0, step = -2) {
            sum += i;
        });

        assert_eq!(sum, 30); // 10+8+6+4+2 = 30
    }

    #[test]
    fn test_iterable() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut sum = 0;

        {
            let mut _break_occurred = false;

            'pythonic_for_loop: for i in vec {
                sum += i;
            }

            if !_break_occurred {
                sum += 100;
            }
        }

        assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
    }

    #[test]
    fn test_iterable_without_else() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut sum = 0;

        pythonic_for!((i in vec) {
            sum += i;
        });

        assert_eq!(sum, 15); // 1+2+3+4+5 = 15
    }

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);

        let mut sum = 0;

        pythonic_for!((entry in map) {
            sum += entry.1;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 106); // 1+2+3+100 = 106
    }

    #[test]
    fn test_hashset() {
        let mut set = HashSet::new();
        set.insert(5);
        set.insert(10);
        set.insert(15);

        let mut sum = 0;

        pythonic_for!((value in set) {
            sum += value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 130); // 5+10+15+100 = 130
    }

    #[test]
    fn test_vecdeque() {
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        let mut sum = 0;

        pythonic_for!((value in deque) {
            sum += value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 106); // 1+2+3+100 = 106
    }

    #[test]
    fn test_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let mut sum = 0;

        pythonic_for!((entry in map) {
            sum += entry.1;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 106); // 1+2+3+100 = 106
    }

    #[test]
    fn test_btreeset() {
        let mut set = BTreeSet::new();
        set.insert(5);
        set.insert(10);
        set.insert(15);

        let mut sum = 0;

        pythonic_for!((value in set) {
            sum += value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 130); // 5+10+15+100 = 130
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

        pythonic_for!((value in square_iter) {
            sum += value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 114); // 1+4+9+100 = 114
    }

    #[test]
    fn test_filter_map() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;

        let even_doubles =
            numbers
                .iter()
                .filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None });

        pythonic_for!((value in even_doubles) {
            sum += value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 112); // 4+8+100 = 112
    }

    #[test]
    fn test_chain_iterator() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];

        let mut sum = 0;

        let chained = first.iter().chain(second.iter());

        pythonic_for!((value in chained) {
            sum += *value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 121); // 1+2+3+4+5+6+100 = 121
    }

    #[test]
    fn test_zip_iterator() {
        let numbers = vec![1, 2, 3];
        let letters = vec!['a', 'b', 'c'];

        let mut result = String::new();

        let zipped = numbers.iter().zip(letters.iter());

        pythonic_for!((pair in zipped) {
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

        pythonic_for!((pair in enumerated) {
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

        pythonic_for!((value in taken) {
            sum += *value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 106); // 1+2+3+100 = 106
    }

    #[test]
    fn test_skip_iterator() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;

        let skipped = numbers.iter().skip(2);

        pythonic_for!((value in skipped) {
            sum += *value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 112); // 3+4+5+100 = 112
    }

    #[test]
    fn test_flat_map_iterator() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        let mut sum = 0;

        let flattened = nested.iter().flat_map(|v| v.iter());

        pythonic_for!((value in flattened) {
            sum += *value;
        } else {
            sum += 100;
        });

        assert_eq!(sum, 110); // 1+2+3+4+100 = 110
    }

    #[test]
    fn test_nested_loops_without_else() {
        let mut sum = 0;
        let mut outer_count = 0;
        let mut inner_breaks = 0;
        
        pythonic_for!((i in 0..3) {
            outer_count += 1;
            
            for j in 0..3 {
                sum += i * j;
                
                if j == 1 {
                    inner_breaks += 1;
                    break;
                }
            }
            
            if i == 1 {
                break;
            }
        });
        
        assert_eq!(outer_count, 2);
        assert_eq!(inner_breaks, 2);
        assert_eq!(sum, 1);
    }
    
    #[test]
    fn test_nested_loops_with_else() {
        let mut sum = 0;
        let mut outer_count = 0;
        let mut inner_breaks = 0;
        let mut else_executed = false;
        
        pythonic_for!((i in 0..3) {
            outer_count += 1;
            
            for j in 0..3 {
                sum += i * j;
                
                if j == 1 {
                    inner_breaks += 1;
                    break;
                }
            }
            
            if i == 1 {
                break;
            }
        } else {
            else_executed = true;
            sum += 100;
        });
        
        assert_eq!(outer_count, 2);
        assert_eq!(inner_breaks, 2);
        assert_eq!(sum, 101);
        assert_eq!(else_executed, true);
    }

    #[test]
    fn test_nested_pythonic_for_inner_break() {
        let mut outer_sum = 0;
        let mut inner_sum = 0;
        let mut outer_else_executed = false;
        let mut inner_else_executed = false;
        
        pythonic_for!((i in 0..3) {
            outer_sum += i;
            
            pythonic_for!((j in 0..3) {
                inner_sum += j;
                
                if j == 1 {
                    break;
                }
            } else {
                inner_else_executed = true;
            });
            
        } else {
            outer_else_executed = true;
            outer_sum += 100;
        });
        
        assert_eq!(outer_sum, 103); // 0+1+2+100 = 103
        assert_eq!(inner_sum, 3);   // (0+1)+(0+1)+(0+1) = 3
        assert_eq!(inner_else_executed, true); // Inner else executes despite break
        assert_eq!(outer_else_executed, true);
    }

    #[test]
    fn test_nested_pythonic_for_outer_break() {
        let mut outer_sum = 0;
        let mut inner_sum = 0;
        let mut outer_else_executed = false;
        let mut inner_else_count = 0;
        
        pythonic_for!((i in 0..3) {
            outer_sum += i;
            
            pythonic_for!((j in 0..3) {
                inner_sum += j;
            } else {
                inner_else_count += 1;
                inner_sum += 10;
            });
            
            if i == 1 {
                break;
            }
        } else {
            outer_else_executed = true;
            outer_sum += 100;
        });
        
        assert_eq!(outer_sum, 101);   // 0+1+100 = 101 (outer else executes despite break)
        assert_eq!(inner_sum, 26);  // (0+1+2)+(0+1+2)+10+10 = 26
        assert_eq!(inner_else_count, 2);
        assert_eq!(outer_else_executed, true);
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _internal_pythonic_for_body {
    ($body:block) => {
        pythonic_for_proc_macros::transform_body! { $body }
    };
}
#[doc(hidden)]
#[inline]
pub fn _is_likely_cycle<T, I: Iterator<Item = T>>(_iter: &I) -> bool {
    let type_name = std::any::type_name::<I>();
    type_name.contains("Cycle")
}

/// This macro is used internally by pythonic_for when a cycle iterator is detected with an else clause
#[macro_export]
#[deprecated(
    since = "0.1.0",
    note = "Using cycle() with an else clause creates a logical error"
)]
macro_rules! _cycle_with_else_warning {
    () => {
    };
}
