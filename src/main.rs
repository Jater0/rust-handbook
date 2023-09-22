use std::env;
mod hello_world;
mod hello_ferris;
mod variable_constants;
mod shadowing;

fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(arg) = args.get(1) {
    match arg.as_str() {
      "1" => handbook_demo_code_1(),
      "2" => handbook_demo_code_2(),
      "3" => handbook_demo_code_3(),
      "4" => handbook_demo_code_4(),
      _ => println!("Unknown")
    }
  } else {
    println!("Unknown");
  }
}

fn handbook_demo_code_1() {
  hello_world::hello();
}

fn handbook_demo_code_2() {
  hello_ferris::hello_from_ferris();
}

fn handbook_demo_code_3() {
  println_divider();
  variable_constants::variable_demo_01();
  println_divider();
  variable_constants::mutable_variable_demo_01();
  println_divider();
}

fn handbook_demo_code_4() {
  println_divider();
  shadowing::shadowing_demo_01();
  println_divider();
  shadowing::shadowing_mutable_variable_demo_01();
  println_divider();
  shadowing::scope_demo_01();
  println_divider();
  shadowing::scope_shadowing_demo_01();
  println_divider();
}

fn println_divider() {
  println!("=============================="); 
}