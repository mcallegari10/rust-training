fn main() {
  // In general, the `{}` will be automatically replaced with any
  // arguments. These will be stringified.
  println!("{} days", 31);

  // Without a suffix, 31 becomes an i32. You can change what type 31 is
  // by providing a suffix. The number 31i64 for example has the type i64.

  // There are various optional patterns this works with. Positional
  // arguments can be used.
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // As can named arguments.
  println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");

  // Special formatting can be specified after a `:`.
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  // You can right-align text with a specified width. This will output
  // "     1". 5 white spaces and a "1".
  println!("{number:>width$}", number=1, width=6);

  // You can pad numbers with extra zeroes. This will output "000001".
  println!("{number:0>width$}", number=1, width=6);

  // Rust even checks to make sure the correct number of arguments are
  // used.
  println!("My name is {0}, {1} {0}", "Bond", "James");

  // Create a structure named `Structure` which contains an `i32`.
  #[derive(Debug)]
  struct Structure(i32);

  #[derive(Debug)]
  struct Deep(Structure);

  // However, custom types such as this structure require more complicated
  // handling. This will not work.
  println!("This struct {:?} will print with derive(Debug)...", Structure(3));
  println!("{:#?} will print with derive(Debug)...", Deep(Structure(100)));

  let pi = 3.141592;
  println!("Pi is roughly {0:.3}", pi);
}
