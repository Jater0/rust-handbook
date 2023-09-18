use std::env;
mod hello_world;
mod hello_ferris;

fn main() {
  let args: Vec<String> = env::args().collect();

  if let Some(arg) = args.get(1) {
    match arg.as_str() {
      "1" => handbook_demo_code_1(),
      "2" => handbook_demo_code_2(),
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
