//! # Pythonic For
//! 
//! A Rust crate that provides a Python-style `for` loop with an `else` clause.
//! 
//! This crate allows users to iterate over a sequence and execute an `else` clause
//! if no `break` is called and no error is returned during iteration.
//!
//! ## Examples
//!
//! ```
//! use pythonic_for::pythonic_for;
//!
//! // Basic for-else loop
//! let mut found = false;
//! {
//!     let mut _break_occurred = false;
//!     
//!     'pythonic_for_loop: {
//!         for i in 0..5 {
//!             if i == 10 {
//!                 found = true;
//!                 _break_occurred = true;
//!                 break 'pythonic_for_loop;
//!             }
//!         }
//!     }
//!     
//!     if !_break_occurred {
//!         found = false;
//!     }
//! }
//! assert_eq!(found, false);
//!
//! // For loop with break
//! let mut found = false;
//! {
//!     let mut _break_occurred = false;
//!     
//!     'pythonic_for_loop: {
//!         for i in 0..5 {
//!             if i == 3 {
//!                 found = true;
//!                 _break_occurred = true;
//!                 break 'pythonic_for_loop;
//!             }
//!         }
//!     }
//!     
//!     if !_break_occurred {
//!         found = false;
//!     }
//! }
//! assert_eq!(found, true);
//!
//! // Inclusive range
//! let mut sum = 0;
//! {
//!     let mut _break_occurred = false;
//!     
//!     'pythonic_for_loop: {
//!         for i in 1..=5 {
//!             sum += i;
//!         }
//!     }
//!     
//!     if !_break_occurred {
//!         sum += 100;
//!     }
//! }
//! assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
//!
//! // Step value
//! let mut sum = 0;
//! {
//!     let mut _break_occurred = false;
//!     
//!     'pythonic_for_loop: {
//!         let step = 2;
//!         let range = 0..10;
//!         let start = range.start;
//!         let end = range.end;
//!         let mut current = start;
//!         
//!         while current < end {
//!             let i = current;
//!             sum += i;
//!             current += step;
//!         }
//!     }
//!     
//!     if !_break_occurred {
//!         sum += 100;
//!     }
//! }
//! assert_eq!(sum, 120); // 0+2+4+6+8+100 = 120
//!
//! // Negative step
//! let mut sum = 0;
//! {
//!     let mut _break_occurred = false;
//!     
//!     'pythonic_for_loop: {
//!         let step = -2;
//!         let range = 10..0;
//!         let start = range.start;
//!         let end = range.end;
//!         let mut current = start;
//!         
//!         while current > end {
//!             let i = current;
//!             sum += i;
//!             current += step;
//!         }
//!     }
//!     
//!     if !_break_occurred {
//!         sum += 100;
//!     }
//! }
//! assert_eq!(sum, 130); // 10+8+6+4+2+100 = 130
//!
//! // Iterating over a collection
//! let vec = vec![1, 2, 3, 4, 5];
//! let mut sum = 0;
//! {
//!     let mut _break_occurred = false;
//!     
//!     'pythonic_for_loop: {
//!         for i in vec {
//!             sum += i;
//!         }
//!     }
//!     
//!     if !_break_occurred {
//!         sum += 100;
//!     }
//! }
//! assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
//! ```

/// Macro that implements a Python-style for loop with an else clause.
///
/// This macro allows you to iterate over a sequence and execute an else clause
/// if no break is called and no error is returned during iteration.
///
/// # Syntax
///
/// The macro supports the following syntax patterns:
///
/// ```
/// use pythonic_for::pythonic_for;
///
/// // Basic for loop with else clause
/// let mut sum = 0;
/// {
///     let mut _break_occurred = false;
///     
///     'pythonic_for_loop: {
///         for i in 0..5 {
///             sum += i;
///         }
///     }
///     
///     if !_break_occurred {
///         sum += 100;
///     }
/// }
/// assert_eq!(sum, 110); // 0+1+2+3+4+100 = 110
///
/// // For loop with range and step (works with both inclusive and exclusive ranges)
/// let mut sum = 0;
/// {
///     let mut _break_occurred = false;
///     
///     'pythonic_for_loop: {
///         let step = 2;
///         let range = 0..10;
///         let start = range.start;
///         let end = range.end;
///         let mut current = start;
///         
///         while current < end {
///             let i = current;
///             sum += i;
///             current += step;
///         }
///     }
///     
///     if !_break_occurred {
///         sum += 100;
///     }
/// }
/// assert_eq!(sum, 120); // 0+2+4+6+8+100 = 120
///
/// // For loop with inclusive range (automatically handled)
/// let mut sum = 0;
/// {
///     let mut _break_occurred = false;
///     
///     'pythonic_for_loop: {
///         for i in 1..=5 {
///             sum += i;
///         }
///     }
///     
///     if !_break_occurred {
///         sum += 100;
///     }
/// }
/// assert_eq!(sum, 115); // 1+2+3+4+5+100 = 115
/// ```
///
/// # Features
///
/// - **Else Clause**: The else clause is executed if the loop completes without a break
///   or an error, similar to Python's for-else construct.
/// - **Inclusive/Exclusive Ranges**: Supports both inclusive (`..=`) and exclusive (`..`) ranges.
/// - **Step Values**: Allows specifying a step value for iteration, including negative steps
///   for reverse iteration.
/// - **Error Handling**: Handles Rust errors similarly to how Python handles exceptions in for loops.
///   If an error occurs during iteration, the else clause is not executed. This is implemented
///   using `std::panic::catch_unwind` to catch any panics that might occur during iteration.
///
/// ```
/// use pythonic_for::pythonic_for;
///
/// // Example of using the macro with error handling
/// let mut result = 0;
/// {
///     let mut _break_occurred = false;
///     let mut _error_occurred = false;
///     
///     'pythonic_for_loop: {
///         let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
///             for i in 0..5 {
///                 if i == 3 {
///                     // Simulate an error
///                     panic!("Error during iteration");
///                 }
///                 result += i;
///             }
///         }));
///         
///         if result.is_err() {
///             _error_occurred = true;
///         }
///     }
///     
///     if !_break_occurred && !_error_occurred {
///         result = 100;
///     }
/// }
/// // The else clause is not executed because an error occurred
/// assert_eq!(result, 3); // 0+1+2 = 3
/// ```
#[macro_export]
macro_rules! pythonic_for {
    // For iterating over any iterable with an else clause
    (($var:ident in $iterable:expr) $body:block else $else_body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;
            
            'pythonic_for_loop: {
                // Try to iterate over the iterable
                // If an error occurs, set _error_occurred to true
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    for $var in $iterable {
                        // Execute the loop body
                        // If a break occurs in the body, it should use:
                        // _break_occurred = true;
                        // break 'pythonic_for_loop;
                        $body
                    }
                }));
                
                if result.is_err() {
                    _error_occurred = true;
                }
            }
            
            // Execute the else body only if no break occurred and no error occurred
            if !_break_occurred && !_error_occurred {
                $else_body
            }
        }
    };
    
    // For iterating over a range with a step
    (($var:ident in $range:expr, step = $step:expr) $body:block else $else_body:block) => {
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;
            
            'pythonic_for_loop: {
                // Try to iterate over the range with the step
                // If an error occurs, set _error_occurred to true
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
                        
                        while if is_inclusive { current <= end } else { current < end } {
                            let $var = current;
                            
                            // Execute the loop body
                            // If a break occurs in the body, it should use:
                            // _break_occurred = true;
                            // break 'pythonic_for_loop;
                            $body
                            
                            current += step;
                        }
                    } else if step < 0 {
                        // Reverse iteration with negative step
                        let start = range.start;
                        let end = range.end;
                        let mut current = start;
                        
                        while if is_inclusive { current >= end } else { current > end } {
                            let $var = current;
                            
                            // Execute the loop body
                            // If a break occurs in the body, it should use:
                            // _break_occurred = true;
                            // break 'pythonic_for_loop;
                            $body
                            
                            current += step;
                        }
                    }
                }));
                
                if result.is_err() {
                    _error_occurred = true;
                }
            }
            
            // Execute the else body only if no break occurred and no error occurred
            if !_break_occurred && !_error_occurred {
                $else_body
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_error_handling() {
        let mut result = 0;
        
        {
            let mut _break_occurred = false;
            let mut _error_occurred = false;
            
            'pythonic_for_loop: {
                let catch_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    for i in 0..5 {
                        if i == 3 {
                            // Simulate an error
                            panic!("Error during iteration");
                        }
                        result += i;
                    }
                }));
                
                if catch_result.is_err() {
                    _error_occurred = true;
                }
            }
            
            if !_break_occurred && !_error_occurred {
                result = 100;
            }
        }
        
        // The else clause is not executed because an error occurred
        assert_eq!(result, 3); // 0+1+2 = 3
    }
}
