extern crate forest_lib;

use forest_lib::forest_array::*;

fn main() {
  let ar_forest = Forest::new( 10, 10, Tree::Tree );
  print!("{}", ar_forest);

}
