
use pythonic_for::pythonic_for;

fn main() {
    println!("Pythonic For Demo");
    println!("=================\n");

    println!("Example 1: Basic for loop without else clause");
    let mut sum = 0;
    pythonic_for!(i in 0..5 {
        println!("  Iteration: i = {}", i);
        sum += i;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 2: Basic for loop with else clause");
    let mut sum = 0;
    pythonic_for!(i in 0..5 {
        println!("  Iteration: i = {}", i);
        sum += i;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 3: For loop with break (else clause not executed)");
    let mut sum = 0;
    pythonic_for!(i in 0..5 {
        println!("  Iteration: i = {}", i);
        sum += i;
        if i == 2 {
            println!("  Breaking at i = 2");
            break;
        }
    } else {
        println!("  Else clause executed (should not see this)");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 4: For loop with step value without else clause");
    let mut sum = 0;
    pythonic_for!(i in 0..10, step = 2 {
        println!("  Iteration: i = {}", i);
        sum += i;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 5: For loop with step value with else clause");
    let mut sum = 0;
    pythonic_for!(i in 0..10, step = 2 {
        println!("  Iteration: i = {}", i);
        sum += i;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 6: For loop with negative step");
    let mut sum = 0;
    pythonic_for!(i in 10..0, step = -2 {
        println!("  Iteration: i = {}", i);
        sum += i;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 7: Iterating over a collection without else clause");
    let vec = vec![1, 2, 3, 4, 5];
    let mut sum = 0;
    pythonic_for!(i in vec {
        println!("  Iteration: i = {}", i);
        sum += i;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 8: Custom iterator");
    let square_iter = SquareIter { current: 1, end: 3 };
    let mut sum = 0;
    pythonic_for!(value in square_iter {
        println!("  Iteration: value = {}", value);
        sum += value;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 9: Iterator adapter - enumerate");
    let letters = vec!['a', 'b', 'c'];
    let mut result = String::new();
    pythonic_for!(pair in letters.iter().enumerate() {
        let (idx, ch) = pair; // This pattern `pair` should be fine as `var_ident`
        println!("  Iteration: idx = {}, ch = {}", idx, ch);
        result.push_str(&format!("{}{}", idx, ch));
    } else {
        println!("  Else clause executed");
        result.push_str("done");
    });
    println!("  Result: {}\n", result);

    println!("Example 10: Iterator adapter - take");
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let mut sum = 0;
    pythonic_for!(n in numbers.iter().take(3) {
        println!("  Iteration: n = {}", n);
        sum += *n; // Assuming n is `&i32` if iter() is used, or `i32` if `into_iter()` implied
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 11: Iterator adapter - skip");
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum = 0;
    pythonic_for!(n in numbers.iter().skip(2) {
        println!("  Iteration: n = {}", n);
        sum += *n;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 12: Iterator adapter - flat_map");
    let nested = vec![vec![1, 2], vec![3, 4]];
    let mut sum = 0;
    pythonic_for!(n in nested.iter().flat_map(|v| v.iter()) {
        println!("  Iteration: n = {}", n);
        sum += *n;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 13: Iterator adapter - cycle with break");
    let numbers = vec![1, 2, 3];
    let mut sum = 0;
    let mut count = 0;
    pythonic_for!(n in numbers.iter().cycle() {
        println!("  Iteration: n = {}", n);
        sum += *n;
        count += 1;
        if count >= 7 {
            println!("  Breaking after 7 iterations");
            break;
        }
    } else {
        println!("  Else clause executed (should not see this)");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 14: Result-based error handling");
    let mut result: Result<i32, &str> = Ok(0);
    pythonic_for!(i in 0..5 {
        println!("  Iteration: i = {}", i);
        if i == 3 {
            println!("  Error at i = 3");
            result = Err("Error during iteration");
            break;
        }
        if let Ok(val) = result {
            result = Ok(val + i);
            println!("  Current result: {:?}", result);
        }
    } else {
        println!("  Else clause executed (should not see this)");
        if let Ok(val) = result {
            result = Ok(val + 100);
        }
    });
    println!("  Final result: {:?}\n", result);

    println!("Example 15: Nested loops");
    let mut sum = 0;
    pythonic_for!(i in 0..3 {
        println!("  Outer loop: i = {}", i);
        
        for j in 0..3 {
            println!("    Inner loop: j = {}", j);
            sum += i * j;
            
            if j == 1 {
                println!("    Breaking inner loop at j = 1");
                break;
            }
        }
        
        if i == 1 {
            println!("  Breaking outer loop at i = 1");
            break;
        }
    } else {
        println!("  Else clause executed (should not see this)");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);
}

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
