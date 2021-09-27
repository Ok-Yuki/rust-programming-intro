enum Event {
    Quit,
    KeyDown(u8),
    MouseDown {x: i32, y: i32},
}

fn main() {
    let e1 = Event::Quit;
    let e2 = Event::MouseDown {x: 10, y: 10};
}
