extern crate forest_lib;

use forest_lib::forest_array::*;
use forest_lib::terminal_display::print_screen;
use forest_lib::game_loop::run;

fn main() {
  let mut ar_forest = Forest::new( 10, 10, Tree::Tree );
  let mut generation = 0;
  let program = || {
    generation += 1;
    ar_forest.update();
    print_screen(&ar_forest, generation);
  };
  run(4, program);
}

