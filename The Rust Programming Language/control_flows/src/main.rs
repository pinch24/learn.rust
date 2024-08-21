fn main() {
    // if 표현식 - 1
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    println!("--------------------------------");

    // if 표현식 - 2
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
    println!("--------------------------------");

    // else if로 여러 조건식 다루기
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("--------------------------------");

    // let 구문에서 if 사용하기 - 1
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {number}");
    println!("--------------------------------");

    // let 구문에서 if 사용하기 - 2
    // let condition = true;
    // let number = if condition { 5 } else { "six" };  // error
    // println!("The value of number is {number}");
    // println!("--------------------------------");

    // loop로 코드 반복하기
    // loop {
    //     println!("again!");  // Infinity Loop
    // }
    // println!("--------------------------------");

    // 반복문에서 값 반환하기
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    println!("--------------------------------");

    // 루프 라벨로 여러 반복문 사이에 모호함 없애기
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("--------------------------------");

    // while을 이용한 조건 반복문
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFEOFF!!!");
    println!("--------------------------------");

    // for를 이용한 컬렉션에 대한 반복문 - 1
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
    println!("--------------------------------");

    // for를 이용한 컬렉션에 대한 반복문 - 2
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}");
    }
    println!("--------------------------------");

    // for를 이용한 컬렉션에 대한 반복문 - 3
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
    println!("--------------------------------");
}
