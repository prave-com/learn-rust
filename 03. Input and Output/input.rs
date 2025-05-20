use std::io;

fn main() {
    let mut input = String::new();

    // Integer input
    println!("Enter a integer:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let number: i32 = input.trim().parse().expect("Please enter a valid integer");

    // Floating point input
    println!("Enter a floating point number:");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let float_number: f64 = input
        .trim()
        .parse()
        .expect("Please enter a valid floating point number");

    // Boolean input
    println!("Enter a boolean value (true/false):");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let boolean_value: bool = input
        .trim()
        .parse()
        .expect("Please enter a 'true' or 'false'");

    // Character input
    println!("Enter a single character:");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let character: char = input
        .trim()
        .chars()
        .next()
        .expect("You must enter a character");

    // String input
    println!("Enter a string:");

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input_string = input.trim();

    // Output the results
    println!("Integer: {}", number);
    println!("Floating point number: {}", float_number);
    println!("Boolean value: {}", boolean_value);
    println!("Character: {}", character);
    println!("String: {}", input_string);
}
