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

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");
}