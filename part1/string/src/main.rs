fn main() {
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    print!("{}\n", &s1);
    print!("{}\n", &s2);
    print!("{}\n", &s3);
}
