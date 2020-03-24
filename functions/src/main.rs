fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
    let y = {
        let z = five() + 3;
        plus_one(z)
    };
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
