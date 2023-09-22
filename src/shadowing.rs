pub fn shadowing_demo_01() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("value of x: {}", x);
}

pub fn shadowing_mutable_variable_demo_01() {
  let x = 1999;
  println!("first value of x: {}", x);
  let x = "Jater Chu";
  println!("second value of x: {}", x);

  let y = 1949;
  println!("value of y: {}", y);
  // Error: mismatched types
  //        expected integer, found `&str`
  // y = "hello";
}

pub fn scope_demo_01() {
  let long_lived = 1;
  {
    let short_lived = 2;
    println!("Inner short_lived: {}", short_lived);
  }

  // Error
  // println!("Outside short_lived: {}", short_lived);

  println!("long_lived: {}", long_lived);
}

pub fn scope_shadowing_demo_01() {
  let shadowing_binding = 1949;
  println!("shadowing_binding-1 {}", shadowing_binding);
  {
    let shadowing_binding = "Jater Chu";
    println!("shadowing_binding-2 {}", shadowing_binding);
  }
  println!("shadowing_binding-3 {}", shadowing_binding);

  let shadowing_binding = 1999;
  println!("shadowing_binding-4 {}", shadowing_binding);
}