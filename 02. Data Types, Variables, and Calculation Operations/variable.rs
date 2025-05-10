const PI: f64 = 3.14159;
const MAX_USERS: u32 = 1000;

static GREETING: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    let x = 5; // Immutable local variable
    let mut y = 10; // Mutable local variable
    y += 5;
    println!("x = {}, y = {}", x, y);

    println!("{}", GREETING);

    unsafe {
        COUNTER += 1;
        println!("COUNTER = {}", COUNTER);
    }
}
