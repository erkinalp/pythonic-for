
use pythonic_for::pythonic_for;

fn main() {
    println!("Pythonic For Demo");
    println!("=================\n");

    println!("Example 1: Basic for loop without else clause");
    let mut sum = 0;
    pythonic_for!((i in 0..5) {
        println!("  Iteration: i = {}", i);
        sum += i;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 2: Basic for loop with else clause");
    let mut sum = 0;
    pythonic_for!((i in 0..5) {
        println!("  Iteration: i = {}", i);
        sum += i;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 3: For loop with break (else clause not executed)");
    let mut sum = 0;
    pythonic_for!((i in 0..5) {
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
    pythonic_for!((i in 0..10, step = 2) {
        println!("  Iteration: i = {}", i);
        sum += i;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 5: For loop with step value with else clause");
    let mut sum = 0;
    pythonic_for!((i in 0..10, step = 2) {
        println!("  Iteration: i = {}", i);
        sum += i;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 6: For loop with negative step");
    let mut sum = 0;
    pythonic_for!((i in 10..0, step = -2) {
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
    pythonic_for!((i in vec) {
        println!("  Iteration: i = {}", i);
        sum += i;
    });
    println!("  Sum: {}\n", sum);

    println!("Example 8: Custom iterator");
    let square_iter = SquareIter { current: 1, end: 3 };
    let mut sum = 0;
    pythonic_for!((value in square_iter) {
        println!("  Iteration: value = {}", value);
        sum += value;
    } else {
        println!("  Else clause executed");
        sum += 100;
    });
    println!("  Sum: {}", sum);
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
