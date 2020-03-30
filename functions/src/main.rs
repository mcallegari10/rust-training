struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    // ownership_example();
    // another_function(5);

    let email = String::from("somemail@example.com");
    let username = String::from("mcallegari10");
    let new_user = create_user(email, username);

    let user2 = User {
        email: String::from("otheremail@example.com"),
        username: String::from("otheruser"),
        ..new_user
    };

    println!("user: {}", new_user.username);
    println!("user2: {}", user2.username);

    let black = Color(0, 0, 0);
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
    
    let y = if x > 0 {
        let z = five() + 3;
        plus_one(z)
    } else {
        10
    };

    println!("Condition true, the value of y is: {}", y);

    let mut times = five();
    let result = loop {
        times -= 1;
        if times == 0 {
            break x * y
        }
    };
    println!("The result is {}", result);
    condition_while();
    get_numbers();
}

fn five() -> i32 {
    5
}

fn ownership_example() {
    let s1 = String::from("Hello world!");
    let s2 = s1.clone();   // .clone is really expensive as it clones all data from the variable
    println!("{}", s1);
    
    let b1 = false;
    let b2 = b1;
    println!("b1: {}", b1); // only types that don't require allocation accept Copy trait

    let (s3, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s3, len);

    let hell = &s3[..4];
    println!("{}", hell);

    let hello = first_word(&s3);
    println!("{}", hello);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn condition_while() {
    // let mut another_number = 3;
    // while another_number != 0 {
    //     println!("{}", another_number);
    //     another_number -= 1;
    // }
    for num in (1..4).rev() {
        println!("{}", num);
    }
    println!("GO!!");
}

fn get_numbers() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
