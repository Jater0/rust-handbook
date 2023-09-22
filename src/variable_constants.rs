pub fn variable_demo_01() {
  let a = 1949;
  println!("{}", a);
  // Error: expected integer, found `&str`
  // a = "hello";
  // Error: expected integer, found floating-point numbe
  // a = 1949.10;
  // Error: cannot assign twice to immutable variable
  // a = 1999;
}

pub fn mutable_variable_demo_01() {
  let a = 10;
  println!("First: {}", a);
  let mut a = 1999;
  println!("Second: {}", a);
  a = 100;
  println!("Third: {}", a);
  let a = "Jater Chu";
  println!("Fourth: {}", a);
}