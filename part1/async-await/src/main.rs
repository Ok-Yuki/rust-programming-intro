use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans1 = async_add(2, 3).await; // この時点で5という値を取り出せる。
    let ans2 = async_add(3, 4).await; // この時点で7という値を取り出せる。
    let ans3 = async_add(4, 5).await; // この時点で9という値を取り出せる。

    let result = ans1 + ans2 + ans3;
    println!("{}", result);
    result
}

fn main() {
    executor::block_on(something_great_async_function());
}
