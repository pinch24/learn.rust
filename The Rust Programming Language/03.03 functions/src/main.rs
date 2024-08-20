fn main() {
    // 매개변수
    another_function(5);
    print_labeled_measurement(5, 'h');
    println!("--------------------------------");

    // 구문과 표현식
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    println!("--------------------------------");

    // 반환 값을 갖는 함수
    let x1 = five();
    println!("The value of x1 is: {x1}");

    let x2 = plus_one(5);
    println!("The value of x2 is: {x2}");
    println!("--------------------------------");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}