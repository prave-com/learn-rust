fn main() {
    // Using println! (with newline)
    let name = "Alice";
    let age = 30;

    println!("Hello, {}! You are {} years old.", name, age);
    println!("Age: {1}, Name: {0}", name, age); // Using index based formatting
    println!("Name: {name}, Age: {age}");
    println!("Name: {abc}, Age: {xyz}", abc = name, xyz = age);

    // Using print! (without newline)
    print!("This is a test, ");
    print!("on the same line!\n");

    // Using eprintln! (to stderr)
    eprintln!("Error: Something went wrong!");

    // Using format! (storing formatted string)
    let formatted_string = format!("Hello, {}! Welcome to Rust programming.", name);
    println!("{}", formatted_string); // Printing the formatted string
}
