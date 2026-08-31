#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Something,
    Move,
    Echo,
    ChangeColor,
    Quit
}

fn main() {
    println!("{:?}", Message::Something);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
