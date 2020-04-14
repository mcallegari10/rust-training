enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
// Message enum is equivalent to the next structs but has the benefit of a single type
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32
// };
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {}
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("some string");
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) {}
