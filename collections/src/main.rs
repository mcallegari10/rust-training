fn main() {
    // let v: Vec<i32> = Vec::new();  -  When you don't have initial values
    let v2 = vec![1, 2]; // use the macro vec! for initial values and infer the type
    
    let mut v = Vec::new(); // Rust infers the type on every push
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    // Accesing vector elements
    let third = &v[2];
    println!("The third element of v is: {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element of v2 is: {}", third),
        None => println!("There is no third element in v2")
    }

    // let _does_not_exist = &v2[100]; -  Will panic and crash
    let _does_not_exist = v2.get(100); // Will return None

    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Multiple type array using enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("black")),
        SpreadsheetCell::Float(10.50)
    ];
    println!("{:?}", row);
}
