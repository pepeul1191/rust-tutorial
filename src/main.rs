#[macro_use] extern crate nickel;

use nickel::Nickel;
mod constants;

fn main() {
  let mut server = Nickel::new();

  server.utilize(router! {
    get "**" => |_req, _res| {
      println!("hola mundo???");
      let mut num: i32 = 10;
      let is_it_true: bool = true;
      let let_x: char = 'x';
      println!("1++++++++++++++++++++");
      println!("{0}, {1}, {0}", num, num+2);
      println!("2++++++++++++++++++++");
      println!("constants");
      println!("3++++++++++++++++++++");
      "Hello world??"
    }
  });

  server.listen("127.0.0.1:6767");
}