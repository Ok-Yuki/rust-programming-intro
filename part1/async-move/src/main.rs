use futures::{Future, executor};

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();

    async move {
        println!("{}", outside_variable);
    }
} 

fn main(){
    move_to_async_block().await;
}
