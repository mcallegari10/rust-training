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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {:?}", state);
            25
        }
    }
}

fn quarter_count(coin: Coin) -> u8 {
    let mut count = 0;
    if Coin::Quarter(state) = coin {
        println!("State Quarter from {:?}", state);
        count
    } else {
        count += 1
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("some string");
    let absent_number: Option<i32> = None;

    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);

    let five = Some(5);
    println!("{:#?}", plus_one(five));
    println!("{:?}", plus_one(None));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
