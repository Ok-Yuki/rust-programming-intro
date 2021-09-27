fn main() {
    let mut count = 0;

    // loopは式のため、値を返すことができる
    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count
        }
    };
    println!("result: {}", result);
}
