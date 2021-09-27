fn main() {
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("num: {}", num);
    }
}

struct Iter {
    current: usize,
    max: usize,
}


// Iteratorとレイトを適用するには、以下2つが必要となる。
// 1. Iteratorが出力する方を決定し、type Itemに紐づける
// 2. 次の要素を返すnext()メソッドを実装する
impl Iterator for Iter {
    type Item = usize; //出力する型の紐付け

    // next()メソッドの実装
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}
