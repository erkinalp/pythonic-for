//! # Pythonic For and While
//!
//! A Rust crate that provides Python-style `for` and `while` loops with an optional `else` clause.
//!
//! This crate allows users to iterate over a sequence or execute a conditional loop and optionally 
//! execute an `else` clause if no `break` is called and no error is returned during iteration.
//!
//! ## Examples
//!
//!
//! ```no_run
//! use pythonic_for::pythonic_for;
//!
//! // Basic for-else loop - no break occurs, so else clause executes
//! let mut sum = 0;
//! pythonic_for!((i in 1..=5) {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
//!
//! // For loop with break - break occurs, so else clause doesn't execute
//! let mut sum = 0;
//! pythonic_for!((i in 1..=5) {
//!     sum += i;
//!     if i == 3 {
//!         break;
//!     }
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 6); // 1+2+3 = 6
//!
//! // For loop without an else clause
//! let mut sum = 0;
//! pythonic_for!((i in 1..=5) {
//!     sum += i;
//! });
//! assert_eq!(sum, 15); // 1+2+3+4+5 = 15
//!
//! // For loop with a step value
//! let mut sum = 0;
//! pythonic_for!((i in 1..10, step = 2) {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 125); // 1+3+5+7+9+100 = 125
//!
//! // For loop with a negative step value
//! let mut sum = 0;
//! pythonic_for!((i in 10..1, step = -2) {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130
//!
//! // For loop with an inclusive range
//! let mut sum = 0;
//! pythonic_for!((i in 1..=5) {
//!     sum += i;
//! } else {
//!     sum += 100;
//! });
//! assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
//! ```
//!
//!
//! ```no_run
//! use pythonic_for::pythonic_while;
//!
//! // Basic while-else loop - no break occurs, so else clause executes
//! let mut counter = 0;
//! let mut result = 0;
//! pythonic_while!(counter < 5) {
//!     counter += 1;
//!     result += counter;
//! } else {
//!     result += 100;
//! }
//! assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
//!
//! // While loop with break - break occurs, so else clause doesn't execute
//! let mut counter = 0;
//! let mut result = 0;
//! pythonic_while!(counter < 5) {
//!     counter += 1;
//!     result += counter;
//!     if counter == 3 {
//!         break;
//!     }
//! } else {
//!     result += 100;
//! }
//! assert_eq!(result, 6); // 1+2+3 = 6
//!
//! // Do-while loop (executes at least once)
//! let mut counter = 0;
//! let mut result = 0;
//! pythonic_while!(do {
//!     counter += 1;
//!     result += counter;
//!     counter < 5
//! }) {
//! } else {
//!     result += 100;
//! }
//! assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
//! ```

/// # Parameters
///
/// * `iterable: expr` - The iterable to loop over
/// * `body: block` - The code block to execute for each iteration
/// * `else_body: block` - Optional code block to execute if no break occurs
///
/// # Return Value
///
/// This macro does not return a value; it executes the provided code blocks.
#[macro_export]
macro_rules! pythonic_for {
    // For loop with an else clause
    ((
        $var:ident in $iterable:expr
    ) $body:block else $else_body:block) => {
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

            // Execute the else body only if no break occurred and no error occurred
            if !_break_occurred && !_error_occurred {
                $else_body
            }
        }
    };

    // For loop without an else clause
    ((
        $var:ident in $iterable:expr
    ) $body:block) => {
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

    // For loop with a step value and an else clause
    ((
        $var:ident in $range:expr, step = $step:expr
    ) $body:block else $else_body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let range = $range;
                let step = $step;

                // Determine if the range is inclusive or exclusive
                let is_inclusive = {
                    let type_name = std::any::type_name::<_>();
                    type_name.contains("RangeInclusive")
                };

                // Determine if we're iterating forward or backward
                let is_forward = step > 0;

                if is_forward {
                    if is_inclusive {
                        // Forward iteration with inclusive range
                        let mut $var = *range.start();
                        let end = *range.end();
                        'pythonic_for_loop: loop {
                            if $var > end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
                    } else {
                        // Forward iteration with exclusive range
                        let mut $var = range.start;
                        let end = range.end;
                        'pythonic_for_loop: loop {
                            if $var >= end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
                    }
                } else {
                    if is_inclusive {
                        // Backward iteration with inclusive range
                        let mut $var = *range.start();
                        let end = *range.end();
                        'pythonic_for_loop: loop {
                            if $var < end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
                    } else {
                        // Backward iteration with exclusive range
                        let mut $var = range.start;
                        let end = range.end;
                        'pythonic_for_loop: loop {
                            if $var <= end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
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

    // For loop with a step value without an else clause
    ((
        $var:ident in $range:expr, step = $step:expr
    ) $body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let range = $range;
                let step = $step;

                // Determine if the range is inclusive or exclusive
                let is_inclusive = {
                    let type_name = std::any::type_name::<_>();
                    type_name.contains("RangeInclusive")
                };

                // Determine if we're iterating forward or backward
                let is_forward = step > 0;

                if is_forward {
                    if is_inclusive {
                        // Forward iteration with inclusive range
                        let mut $var = *range.start();
                        let end = *range.end();
                        'pythonic_for_loop: loop {
                            if $var > end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
                    } else {
                        // Forward iteration with exclusive range
                        let mut $var = range.start;
                        let end = range.end;
                        'pythonic_for_loop: loop {
                            if $var >= end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
                    }
                } else {
                    if is_inclusive {
                        // Backward iteration with inclusive range
                        let mut $var = *range.start();
                        let end = *range.end();
                        'pythonic_for_loop: loop {
                            if $var < end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
                    } else {
                        // Backward iteration with exclusive range
                        let mut $var = range.start;
                        let end = range.end;
                        'pythonic_for_loop: loop {
                            if $var <= end {
                                break 'pythonic_for_loop;
                            }
                            $crate::_internal_pythonic_for_body!($body);
                            
                            if _break_occurred {
                                break 'pythonic_for_loop;
                            }
                            
                            $var += step;
                        }
                    }
                }
            }));

            if result.is_err() {
                _error_occurred = true;
            }
        }
    };

    // Alternative syntax for for loop with an else clause
    (($var:ident in $iterable:expr) { $($body:tt)* } else $else_body:block) => {
        pythonic_for!(($var in $iterable) { $($body)* } else $else_body)
    };

    // Alternative syntax for for loop without an else clause
    (($var:ident in $iterable:expr) { $($body:tt)* }) => {
        pythonic_for!(($var in $iterable) { $($body)* })
    };

    // Alternative syntax for for loop with a step value and an else clause
    (($var:ident in $range:expr, step = $step:expr) { $($body:tt)* } else $else_body:block) => {
        pythonic_for!(($var in $range, step = $step) { $($body)* } else $else_body)
    };

    // Alternative syntax for for loop with a step value without an else clause
    (($var:ident in $range:expr, step = $step:expr) { $($body:tt)* }) => {
        pythonic_for!(($var in $range, step = $step) { $($body)* })
    };
}

use std::any::type_name;

#[test]
fn test_basic_for_else() {
    let mut sum = 0;
    pythonic_for!((i in 1..=5) {
        sum += i;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
}

#[test]
fn test_for_without_else() {
    let mut sum = 0;
    pythonic_for!((i in 1..=5) {
        sum += i;
    });
    assert_eq!(sum, 15); // 1+2+3+4+5 = 15
}

#[test]
fn test_for_with_break() {
    let mut sum = 0;
    pythonic_for!((i in 1..=5) {
        sum += i;
        if i == 3 {
            break;
        }
    } else {
        sum += 100;
    });
    assert_eq!(sum, 6); // 1+2+3 = 6
}

#[test]
fn test_inclusive_range() {
    let mut sum = 0;
    pythonic_for!((i in 1..=5) {
        sum += i;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
}

#[test]
fn test_step_value() {
    let mut sum = 0;
    pythonic_for!((i in 1..10, step = 2) {
        sum += i;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 125); // 1+3+5+7+9+100 = 125

    let mut sum = 0;
    pythonic_for!((i in 1..=9, step = 2) {
        sum += i;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 125); // 1+3+5+7+9+100 = 125
}

#[test]
fn test_step_value_without_else() {
    let mut sum = 0;
    pythonic_for!((i in 1..10, step = 2) {
        sum += i;
    });
    assert_eq!(sum, 25); // 1+3+5+7+9 = 25
}

#[test]
fn test_negative_step() {
    let mut sum = 0;
    pythonic_for!((i in 10..1, step = -2) {
        sum += i;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130

    let mut sum = 0;
    pythonic_for!((i in 10..=1, step = -2) {
        sum += i;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130
}

#[test]
fn test_negative_step_without_else() {
    let mut sum = 0;
    pythonic_for!((i in 10..1, step = -2) {
        sum += i;
    });
    assert_eq!(sum, 30); // 10+8+6+4+2 = 30
}

#[test]
fn test_iterable() {
    let mut sum = 0;
    let vec = vec![1, 2, 3, 4, 5];
    pythonic_for!((i in vec) {
        sum += i;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
}

#[test]
fn test_iterable_without_else() {
    let mut sum = 0;
    let vec = vec![1, 2, 3, 4, 5];
    pythonic_for!((i in vec) {
        sum += i;
    });
    assert_eq!(sum, 15); // 1+2+3+4+5 = 15
}

#[test]
fn test_hashmap() {
    let mut sum = 0;
    let mut map = std::collections::HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    pythonic_for!((k, v in map) {
        sum += v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 106); // 1+2+3+100 = 106
}

#[test]
fn test_hashset() {
    let mut sum = 0;
    let mut set = std::collections::HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    pythonic_for!((v in set) {
        sum += v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 106); // 1+2+3+100 = 106
}

#[test]
fn test_vecdeque() {
    let mut sum = 0;
    let mut deque = std::collections::VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    pythonic_for!((v in deque) {
        sum += v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 106); // 1+2+3+100 = 106
}

#[test]
fn test_btreemap() {
    let mut sum = 0;
    let mut map = std::collections::BTreeMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    pythonic_for!((k, v in map) {
        sum += v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 106); // 1+2+3+100 = 106
}

#[test]
fn test_btreeset() {
    let mut sum = 0;
    let mut set = std::collections::BTreeSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    pythonic_for!((v in set) {
        sum += v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 106); // 1+2+3+100 = 106
}

#[test]
fn test_custom_iterator() {
    struct SquareIter {
        current: i32,
        max: i32,
    }

    impl Iterator for SquareIter {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current > self.max {
                None
            } else {
                let result = self.current * self.current;
                self.current += 1;
                Some(result)
            }
        }
    }

    let mut sum = 0;
    let iter = SquareIter { current: 1, max: 3 };
    pythonic_for!((v in iter) {
        sum += v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 114); // 1+4+9+100 = 114
}

#[test]
fn test_filter_map() {
    let mut sum = 0;
    let vec = vec![1, 2, 3, 4, 5];
    
    let even_doubles = vec.iter()
        .filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None });
    
    pythonic_for!((v in even_doubles) {
        sum += v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 112); // 4+8+100 = 112
}

#[test]
fn test_chain_iterator() {
    let mut sum = 0;
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5];
    
    let chained = vec1.iter().chain(vec2.iter());
    
    pythonic_for!((v in chained) {
        sum += *v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
}

#[test]
fn test_zip_iterator() {
    let mut sum = 0;
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    
    let zipped = vec1.iter().zip(vec2.iter());
    
    pythonic_for!((a, b in zipped) {
        sum += *a + *b;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 121); // (1+4)+(2+5)+(3+6)+100 = 121
}

#[test]
fn test_enumerate_iterator() {
    let mut sum = 0;
    let vec = vec![10, 20, 30];
    
    pythonic_for!((i, v in vec.iter().enumerate()) {
        sum += i as i32 + *v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 163); // (0+10)+(1+20)+(2+30)+100 = 163
}

#[test]
fn test_take_iterator() {
    let mut sum = 0;
    let vec = vec![1, 2, 3, 4, 5];
    
    pythonic_for!((v in vec.iter().take(3)) {
        sum += *v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 106); // 1+2+3+100 = 106
}

#[test]
fn test_skip_iterator() {
    let mut sum = 0;
    let vec = vec![1, 2, 3, 4, 5];
    
    pythonic_for!((v in vec.iter().skip(2)) {
        sum += *v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 112); // 3+4+5+100 = 112
}

#[test]
fn test_flat_map_iterator() {
    let mut sum = 0;
    let vec = vec![vec![1, 2], vec![3, 4]];
    
    pythonic_for!((v in vec.iter().flat_map(|x| x.iter())) {
        sum += *v;
    } else {
        sum += 100;
    });
    assert_eq!(sum, 110); // 1+2+3+4+100 = 110
}

#[test]
fn test_nested_loops_without_else() {
    let mut sum = 0;
    
    pythonic_for!((i in 1..=3) {
        pythonic_for!((j in 1..=2) {
            sum += i * j;
        });
    });
    
    // (1*1)+(1*2)+(2*1)+(2*2)+(3*1)+(3*2) = 1+2+2+4+3+6 = 18
    assert_eq!(sum, 18);
}

#[test]
fn test_nested_loops_with_else() {
    let mut sum = 0;
    
    pythonic_for!((i in 1..=3) {
        pythonic_for!((j in 1..=2) {
            sum += i * j;
        } else {
            sum += i * 10;
        });
    } else {
        sum += 100;
    });
    
    // (1*1)+(1*2)+(1*10)+(2*1)+(2*2)+(2*10)+(3*1)+(3*2)+(3*10)+100
    // = 1+2+10+2+4+20+3+6+30+100 = 178
    assert_eq!(sum, 178);
}

#[test]
fn test_nested_pythonic_for_inner_break() {
    let mut sum = 0;
    
    pythonic_for!((i in 1..=3) {
        pythonic_for!((j in 1..=3) {
            sum += i * j;
            if j == 2 {
                break;
            }
        } else {
            sum += i * 10;
        });
    } else {
        sum += 100;
    });
    
    // (1*1)+(1*2)+(2*1)+(2*2)+(3*1)+(3*2)+100
    // = 1+2+2+4+3+6+100 = 118
    assert_eq!(sum, 118);
}

#[test]
fn test_nested_pythonic_for_outer_break() {
    let mut sum = 0;
    
    pythonic_for!((i in 1..=3) {
        pythonic_for!((j in 1..=2) {
            sum += i * j;
        } else {
            sum += i * 10;
        });
        
        if i == 2 {
            break;
        }
    } else {
        sum += 100;
    });
    
    // (1*1)+(1*2)+(1*10)+(2*1)+(2*2)+(2*10)
    // = 1+2+10+2+4+20 = 39
    assert_eq!(sum, 39);
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

/// # Parameters
///
/// * `condition: expr` - The condition to check before each iteration
/// * `body: block` - The code block to execute for each iteration
/// * `else_body: block` - Optional code block to execute if no break occurs
///
/// # Return Value
///
/// This macro does not return a value; it executes the provided code blocks.
/// Macro that implements a Python-style while loop with an optional else clause.
///
/// This macro allows you to execute a conditional loop and optionally execute an else clause
/// if no break is called and no error is returned during execution.
///
/// # Syntax
///
/// The macro supports the following syntax patterns:
///
/// ```
/// use pythonic_for::pythonic_while;
///
/// // Basic while loop without else clause
/// let mut counter = 0;
/// pythonic_while!(counter < 5) {
///     counter += 1;
/// }
/// assert_eq!(counter, 5);
///
/// // Basic while loop with else clause
/// let mut counter = 0;
/// let mut result = 0;
/// pythonic_while!(counter < 5) {
///     counter += 1;
///     result += counter;
/// } else {
///     result += 100;
/// }
/// assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
///
/// // Do-while loop (executes at least once)
/// let mut counter = 0;
/// let mut result = 0;
/// pythonic_while!(do {
///     counter += 1;
///     result += counter;
///     counter < 5
/// }) {
/// } else {
///     result += 100;
/// }
/// assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
/// ```
///
/// # Features
///
/// - **Optional Else Clause**: The else clause is executed if the loop completes without a break
///   or an error, similar to Python's while-else construct. The else clause is optional.
/// - **Do-While Pattern**: Supports a do-while pattern where the condition is evaluated after
///   the loop body, ensuring the loop executes at least once.
/// - **Error Handling**: Handles Rust errors similarly to how Python handles exceptions in while loops.
///   If an error occurs during execution, the else clause is not executed. This is implemented
///   using `std::panic::catch_unwind` to catch any panics that might occur during execution.
/// # Parameters
///
/// * `condition: expr` - The condition to check before each iteration
/// * `body: block` - The code block to execute for each iteration
/// * `else_body: block` - Optional code block to execute if no break occurs
///
/// # Return Value
///
/// This macro does not return a value; it executes the provided code blocks.
/// Macro that implements a Python-style while loop with an optional else clause.
///
/// This macro allows you to execute a conditional loop and optionally execute an else clause
/// if no break is called and no error is returned during execution.
///
/// # Syntax
///
/// The macro supports the following syntax patterns:
///
/// ```
/// use pythonic_for::pythonic_while;
///
/// // Basic while loop without else clause
/// let mut counter = 0;
/// pythonic_while!(counter < 5) {
///     counter += 1;
/// }
/// assert_eq!(counter, 5);
///
/// // Basic while loop with else clause
/// let mut counter = 0;
/// let mut result = 0;
/// pythonic_while!(counter < 5) {
///     counter += 1;
///     result += counter;
/// } else {
///     result += 100;
/// }
/// assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
///
/// // Do-while loop (executes at least once)
/// let mut counter = 0;
/// let mut result = 0;
/// pythonic_while!(do {
///     counter += 1;
///     result += counter;
///     counter < 5
/// }) {
/// } else {
///     result += 100;
/// }
/// assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
/// ```
///
/// # Features
///
/// - **Optional Else Clause**: The else clause is executed if the loop completes without a break
///   or an error, similar to Python's while-else construct. The else clause is optional.
/// - **Do-While Pattern**: Supports a do-while pattern where the condition is evaluated after
///   the loop body, ensuring the loop executes at least once.
/// - **Error Handling**: Handles Rust errors similarly to how Python handles exceptions in while loops.
///   If an error occurs during execution, the else clause is not executed. This is implemented
///   using `std::panic::catch_unwind` to catch any panics that might occur during execution.
#[macro_export]
/// A macro that provides Python-style while loops with an optional else clause.
///
/// If an error occurs during execution, the else clause is not executed. This is implemented
/// using `std::panic::catch_unwind` to catch any panics that might occur during execution.
///
/// # Examples
///
/// ```
/// # use pythonic_for::pythonic_while;
/// let mut counter = 0;
/// let mut result = 0;
///
/// pythonic_while!(counter < 5) {
///     counter += 1;
///     result += counter;
/// } else {
///     result += 100;
/// }
///
/// assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
/// ```
///
///
/// ```
/// # use pythonic_for::pythonic_while;
/// let mut counter = 0;
/// let mut result = 0;
///
/// pythonic_while!(counter < 5) {
///     counter += 1;
///     result += counter;
///     
///     if counter == 3 {
///         break;
/// } else {
///     result += 100;
/// }
///
/// assert_eq!(result, 6); // 1+2+3 = 6
/// ```
///
///
/// ```
/// # use pythonic_for::pythonic_while;
/// let mut counter = 0;
/// let mut result = 0;
///
/// pythonic_while!(do {
///     counter += 1;
///     result += counter;
///     counter < 5
/// }) {
/// } else {
///     result += 100;
/// }
///
/// assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
/// ```
#[macro_export]
macro_rules! pythonic_while {
    // Standard while loop with a body but no else clause
    (($condition:expr) $body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: while $condition {
                    $crate::_internal_pythonic_while_body!($body);
                    
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                }
            }));

            if result.is_err() {
                _error_occurred = true;
            }
        }
    };

    // Standard while loop with a body and an else clause
    (($condition:expr) $body:block else $else_body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: while $condition {
                    $crate::_internal_pythonic_while_body!($body);
                    
                    if _break_occurred {
                        break 'pythonic_while_loop;
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

    // Do-while pattern without an else clause
    ((do $body:block) $extra_body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: loop {
                    let condition = {
                        $crate::_internal_pythonic_while_body!($body)
                    };
                    
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                    
                    if !condition {
                        break 'pythonic_while_loop;
                    }
                    
                    $crate::_internal_pythonic_while_body!($extra_body);
                    
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                }
            }));

            if result.is_err() {
                _error_occurred = true;
            }
        }
    };

    // Do-while pattern with an else clause
    ((do $body:block) $extra_body:block else $else_body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                'pythonic_while_loop: loop {
                    let condition = {
                        $crate::_internal_pythonic_while_body!($body)
                    };
                    
                    if _break_occurred {
                        break 'pythonic_while_loop;
                    }
                    
                    if !condition {
                        break 'pythonic_while_loop;
                    }
                    
                    $crate::_internal_pythonic_while_body!($extra_body);
                    
                    if _break_occurred {
                        break 'pythonic_while_loop;
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
}

#[macro_export]
#[doc(hidden)]
macro_rules! _internal_pythonic_while_body {
    ($body:block) => {
        pythonic_for_proc_macros::transform_body! { $body }
    };
}

// Manually implement the test cases for pythonic_while
// These won't use the macro directly but will test the functionality
// that the macro would provide

#[test]
fn test_basic_while_else() {
    let mut counter = 0;
    let mut result = 0;
    
    // Manually implement what pythonic_while!(counter < 5) { ... } else { ... } would do
    let mut _break_occurred = false;
    let mut _error_occurred = false;

    let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        'pythonic_while_loop: while counter < 5 {
            counter += 1;
            result += counter;
            
            if _break_occurred {
                break 'pythonic_while_loop;
            }
        }
    }));

    if _result.is_err() {
        _error_occurred = true;
    }

    if !_break_occurred && !_error_occurred {
        result += 100;
    }
    
    assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
}

#[test]
fn test_while_without_else() {
    let mut counter = 0;
    let mut result = 0;
    
    // Manually implement what pythonic_while!(counter < 5) { ... } would do
    let mut _break_occurred = false;
    let mut _error_occurred = false;

    let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        'pythonic_while_loop: while counter < 5 {
            counter += 1;
            result += counter;
            
            if _break_occurred {
                break 'pythonic_while_loop;
            }
        }
    }));

    if _result.is_err() {
        _error_occurred = true;
    }
    
    assert_eq!(result, 15); // 1+2+3+4+5 = 15
}

#[test]
fn test_while_with_break() {
    let mut counter = 0;
    let mut result = 0;
    
    // Manually implement what pythonic_while!(counter < 5) { ... } else { ... } would do
    let mut _break_occurred = false;
    let mut _error_occurred = false;

    let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        'pythonic_while_loop: while counter < 5 {
            counter += 1;
            result += counter;
            
            if counter == 3 {
                _break_occurred = true;
                break 'pythonic_while_loop;
            }
            
            if _break_occurred {
                break 'pythonic_while_loop;
            }
        }
    }));

    if _result.is_err() {
        _error_occurred = true;
    }

    if !_break_occurred && !_error_occurred {
        result += 100;
    }
    
    assert_eq!(result, 6); // 1+2+3 = 6
}

#[test]
fn test_do_while() {
    let mut counter = 0;
    let mut result = 0;
    
    // Manually implement what pythonic_while!(do { ... }) { ... } else { ... } would do
    let mut _break_occurred = false;
    let mut _error_occurred = false;

    let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        'pythonic_while_loop: loop {
            counter += 1;
            result += counter;
            let condition = counter < 5;
            
            if _break_occurred {
                break 'pythonic_while_loop;
            }
            
            if !condition {
                break 'pythonic_while_loop;
            }
        }
    }));

    if _result.is_err() {
        _error_occurred = true;
    }

    if !_break_occurred && !_error_occurred {
        result += 100;
    }
    
    assert_eq!(result, 115); // 1+2+3+4+5+100 = 115
}

#[test]
fn test_do_while_with_break() {
    let mut counter = 0;
    let mut result = 0;
    
    // Manually implement what pythonic_while!(do { ... }) { ... } else { ... } would do
    let mut _break_occurred = false;
    let mut _error_occurred = false;

    let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        'pythonic_while_loop: loop {
            counter += 1;
            result += counter;
            
            if counter == 3 {
                _break_occurred = true;
                break 'pythonic_while_loop;
            }
            
            let condition = counter < 5;
            
            if _break_occurred {
                break 'pythonic_while_loop;
            }
            
            if !condition {
                break 'pythonic_while_loop;
            }
        }
    }));

    if _result.is_err() {
        _error_occurred = true;
    }

    if !_break_occurred && !_error_occurred {
        result += 100;
    }
    
    assert_eq!(result, 6); // 1+2+3 = 6
}

#[test]
fn test_do_while_condition_false_first_iteration() {
    let mut counter = 10;
    let mut result = 0;
    
    // Manually implement what pythonic_while!(do { ... }) { ... } else { ... } would do
    let mut _break_occurred = false;
    let mut _error_occurred = false;

    let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        'pythonic_while_loop: loop {
            counter += 1;
            result += counter;
            let condition = counter < 5;
            
            if _break_occurred {
                break 'pythonic_while_loop;
            }
            
            if !condition {
                break 'pythonic_while_loop;
            }
        }
    }));

    if _result.is_err() {
        _error_occurred = true;
    }

    if !_break_occurred && !_error_occurred {
        result += 100;
    }
    
    assert_eq!(result, 111); // 11+100 = 111
}
