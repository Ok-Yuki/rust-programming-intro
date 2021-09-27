
// and_then()用の関数
fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
}

fn main() {
    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    };

    let result: Result<i32, String> = Ok(200);
    
    if let Ok(code) = result {
        println!("code: {}", code);
    }

    // Result型をmatch、if letで記述すると、可読性が低下する恐れがある。
    // それを回避するための記述方法が用意されている。

    // unwrap_or: Okの場合は中の値を返却し、Errの場合は引数の値を返却する。
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));

    // and_then: Okの場合にのみ指定した関数を実行する。
    let result: Result<i32, String> = Ok(200);
    let _ = result.and_then(func); // funcは実行される。
    let result: Result<i32, String> = Err("error".to_string());
    let _ = result.and_then(func); // funcは実行されない。
}
