fn main() {
    // loop、while、forにおいて
    // ラベルを指定してbreakすることが可能
    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;

            println!("sub loop end");
        }
        println!("main loop end");
    }
}
