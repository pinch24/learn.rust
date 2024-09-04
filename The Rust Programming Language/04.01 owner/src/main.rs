fn main() {
    // 변수의 스코프
    {                           // s는 아직 선언되지 않아서 여기서는 유효하지 않다.
        let s = "hello";  // 이 지점부터 s는 유효하다.
        // ...                  // s로 어떤 작업을 한다.
    }                           // 이 스코프가 종료되었고, s는 더 이상 유효하지 않다.
    println!("--------------------------------");

    // String 타입 - 1
    let s = String::from("Hello");
    println!("--------------------------------");

    // String 타입 - 2
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
    println!("--------------------------------");

    // 변수와 데이터 간 상호작동 방식 - 이동 1
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
    println!("--------------------------------");

    // 변수와 데이터 간 상호작동 방식 - 이동 2
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1);   // Error: borrow of moved value: `s1`
    println!("{}, world!", s2);
    println!("--------------------------------");

    // 변수와 데이터 간 상호작동 방식 - 클론
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("--------------------------------");

    // 소유권과 함수
    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}", s);  // Error: borrow of moved value: `s`
    let x = 5;
    makes_copy(x);
    println!("{}", x);
    println!("--------------------------------");

    // 반환 값과 스코프 - 1
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");
    println!("{}", s2);

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
    //println!("{}", s2); // Error: borrow of moved value: `s2`
    println!("--------------------------------");

    // 반환 값과 스코프 - 2
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    println!("--------------------------------");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
