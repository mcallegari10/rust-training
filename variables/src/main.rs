fn main() {
    let some_num = 5;
    let some_num = some_num + 1;
    let some_num = some_num * 2;
    println!("The value of some_num is {}", some_num);

    let tup : (i32, f64, u8) = (500, 5.0, 1);
    let (x, y, z) = tup; // Accessing via destructuring
    let five_hundred = tup.0; // Direct access
    println!("The value of y is {}", y);
}
