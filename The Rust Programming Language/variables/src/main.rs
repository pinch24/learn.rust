const PI: f64 = std::f64::consts::PI;   // 3.141592653589793
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    // 10,000s --> 3 Hours

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("--------------------------------");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");
    println!("--------------------------------");

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("--------------------------------");

    let bNumber = 0b1111_0000;
    println!("b Number: {bNumber}");

    let bChar = b'A';
    println!("b Char: {bChar}");
    println!("--------------------------------");

    let x = 2.0;
    let y: f32 = 3.0;
    println!("{x}, {y}");
    println!("--------------------------------");
}