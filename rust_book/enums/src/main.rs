enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}