enum Color {
    Red,
    Blue,
    Green,
}


fn main() {
    let i: i32 = 1;

    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"), // アンダースコアは、あらゆる値にマッチする。
    }
    // 列挙型をmatchで使用する際は、足りない列挙子がある場合、コンパイルエラーになる。
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    // matchも式のため、値を返却することが可能
    let result: Result<i32, String> = Ok(200);

    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };
    println!("result_number: {}", result_number);
}
