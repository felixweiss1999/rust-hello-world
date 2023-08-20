#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String),
}

#[derive(PartialEq)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        if let Message::Write(_) = *self {println!("I'm a message!")}
        else {println!("I'm not a message!")}
    }
}



fn main() {
    let ipversion = IpAddrKind::V4(255, 1, 0, 0);
    let ipversion6 = IpAddrKind::V6(String::from(""));
    dbg!(ipversion);
    dbg!(ipversion6);
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Quit;
    m.call();
}