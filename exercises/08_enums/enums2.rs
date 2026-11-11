#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize(Rectangle),
    Move(Point),
    Echo(String),
    ChangeColor((u8, u8, u8)),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize (Rectangle { width: 10, height: 20 }),
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor((200, 255, 255)),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
