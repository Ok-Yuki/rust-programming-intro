pub fn add(x:i32, y:i32) -> i32 {
    return x + y;
}


#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(1, add(1, 0));
    assert_eq!(2, add(1, 1));
}

#[test]
fn assert_ample() {
    // 第一引数がfalseの場合にpanic!
    assert!(true);
    assert!(false, "panic! value={}", false);

    // 2つの値が異なる場合にpanic!
    assert_eq!(true, true);
    // 2つの値が同じ場合にpanic!
    assert_ne!(true, false);
    
    assert_eq!(true, false, "panic! value({}, {})", true, false);
}

//panic!が発生することを想定するテスト
#[test]
#[should_panic]
fn test_panic() {
    panic!("expected panic");
}

// 通常のcargo testでは実行されない、
// cargo test -- --ignoredで実行される
#[test]
#[ignore]
fn test_add_ignored() {
    assert_eq!(-2, add(-1, -1));
}