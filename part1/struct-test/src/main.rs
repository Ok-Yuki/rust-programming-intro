struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: String::from("John"),
        age: 8,
    };
    println!("name={}\nage={}", p.name, p.age);
}