//　マルチスレッド環境でデータ共有はArcを使用する。
// 排他制御を行う構造はMutexで提供している。
use std::sync::{Arc, Mutex};
// シングルスレッド環境でデータ共有はRcを使用する。
// use std::rc::Rc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            // lockを使ってdataへの可変参照を得る。
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
}
