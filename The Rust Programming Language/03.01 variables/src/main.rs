use std::io;

// 상수
const PI: f64 = std::f64::consts::PI;   // 3.141592653589793
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    // 10,000s --> 3 Hours

fn main() {
    // 새도잉 1
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    println!("--------------------------------");

    // 새도잉 2
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");
    println!("--------------------------------");

    // 정적 데이터 타입
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("--------------------------------");

    // 스칼라 타입
    let number = 0b1111_0000;
    println!("Number: {number}");

    let char = b'A';
    println!("char: {char}");
    println!("--------------------------------");

    // 부동 소수점 타입
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{x}, {y}");
    println!("--------------------------------");

    // 수치 연산
    let sum = 5 + 10;
    let difference = 99.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("sum = {sum}");
    println!("difference = {difference}");
    println!("product = {product}");
    println!("quotient = {quotient}");
    println!("truncated = {truncated}");
    println!("remainder = {remainder}");
    println!("--------------------------------");

    // 부울린 타입
    let t = true;
    let f: bool = false;
    println!("{t}, {f}");
    println!("--------------------------------");

    // 문자 타입
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");
    println!("--------------------------------");

    // 튜플 타입
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (t1, t2, t3) = tup;
    println!("{t1}, {t2}, {t3}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}, {six_point_four}, {one}");
    println!("--------------------------------");

    // 배열 타입
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("{:?}", months);

    let a1 = [1, 2, 3, 4, 5];
    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 5];
    println!("{:?}, {:?}, {:?}", a1, a2, a3);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");
    println!("--------------------------------");

    // 유효하지 않은 배열 요소에 대한 접근
    let array = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];
    println!("The value of the element at index {index} is: {element}");
}