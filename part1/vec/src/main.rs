fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    println!("{}", v1[0]);

    println!("{:?}", v1.pop());
    println!("{:?}", v1);

    let v2 = vec![0; 5];
    for element in &v2 {
        println!("{}", element);
    }
}
