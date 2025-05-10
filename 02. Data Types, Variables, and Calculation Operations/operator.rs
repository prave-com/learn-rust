fn main() {
    // Assignment operator
    let mut x = 20;
    let y = 6;

    println!("Initial values: x = {}, y = {}", x, y);

    // Arithmetic operations
    let addition = x + y;
    let subtraction = x - y;
    let multiplication = x * y;
    let division = x / y;
    let modulus = x % y;

    println!("\nArithmetic Operations:");
    println!("x + y = {}", addition);
    println!("x - y = {}", subtraction);
    println!("x * y = {}", multiplication);
    println!("x / y = {}", division);
    println!("x % y = {}", modulus);

    // Compound assignment operators
    x += 5; // x = x + 5
    println!("\nAfter x += 5: x = {}", x);

    x -= 2; // x = x - 2
    println!("After x -= 2: x = {}", x);

    x *= 3; // x = x * 3
    println!("After x *= 3: x = {}", x);

    x /= 4; // x = x / 4
    println!("After x /= 4: x = {}", x);

    x %= 7; // x = x % 7
    println!("After x %= 7: x = {}", x);
}
