fn main() {
    another_function(5);
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
