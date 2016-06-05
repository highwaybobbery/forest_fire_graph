mod forest;
mod terminal_display;
mod game_loop;

use terminal_display::print_screen;
use game_loop::run;

use std::thread;
use std::sync::{Arc, Mutex};

use forest::Updatable;
use forest::forest_array::Forest;
use forest::tree::Tree;


fn main() {
  let forest : Arc<Mutex<Forest<Tree>>> =
    Arc::new(Mutex::new(Forest::new(40, 20)));

  std::thread::sleep(std::time::Duration::from_millis(1000));
  print_screen(&*forest.lock().unwrap());
  std::thread::sleep(std::time::Duration::from_millis(1000));

  let c_forest = forest.clone();
  let _ = thread::spawn(move || {
    loop {
      {
        let mut c_forest = c_forest.lock().unwrap();
        c_forest.update();
      }
    }
  });

  run(5, ||{
    print_screen(&*forest.lock().unwrap());
  });
}
