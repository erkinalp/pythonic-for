use pythonic_for::{pythonic_for, pythonic_while};

fn main() {
    let mut result1 = 0;
    
    pythonic_while!(false; {
        result1 += 10; // This will never execute
    } else {
        result1 += 100; // This will always execute
    });
    
    println!("Test 1 result: {}", result1); // Should be 100
    
    let mut result2 = 0;
    
    pythonic_while!(0 == 1; {
        result2 += 20; // This will never execute
    } else {
        result2 += 200; // This will always execute
    });
    
    println!("Test 2 result: {}", result2); // Should be 200
    
    let mut result3 = 0;
    
    pythonic_while!(10 < 5; {
        result3 += 30; // This will never execute
    } else {
        result3 += 300; // This will always execute
    });
    
    println!("Test 3 result: {}", result3); // Should be 300
    
    let x = 10;
    let y = 5;
    let mut result4 = 0;
    
    pythonic_while!(x < y; {
        result4 += 40; // This will never execute
    } else {
        result4 += 400; // This will always execute
    });
    
    println!("Test 4 result: {}", result4); // Should be 400
    
    let mut counter = 0;
    let mut result5 = 0;
    
    pythonic_while!(counter < 3; {
        counter += 1;
        result5 += counter * 10;
    } else {
        result5 += 500;
    });
    
    println!("Test 5 result: {}", result5); // Should be 60 (10+20+30) + 500 = 560
}
