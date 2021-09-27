
// implキーワードを使う事によって、構造体にメソッドを加えることができます。
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 関連関数　型から関数を呼ぶ形式（staticメソッド的なやつ）
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: String::from(name),
            age: age,
        }
    }

    fn say_name(&self) -> &Self{
        println!("I am {}.", self.name);
        self //戻り値に自分自身を指定することで、メソッドチェーンを使うことができる。
    }

    fn say_age(&self) -> &Self{
        println!("I am {} year(s) old.", self.age);
        self //戻り値に自分自身を指定することで、メソッドチェーンを使うことができる。
    }
}

fn main() {
    let p = Person::new("Taro", 20);

    p.say_name().say_age();
}
