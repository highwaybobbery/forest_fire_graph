use std::time::Duration;
use std::thread::sleep;

pub fn run<F>(sleep_millis: u64, mut program: F) where F : FnMut() {
  let sleep_duration = Duration::from_millis(sleep_millis);
  loop {
    program();
    sleep(sleep_duration);
  }
}
