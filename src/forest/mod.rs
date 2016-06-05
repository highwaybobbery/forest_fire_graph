pub mod forest_array;
pub mod tree;

pub trait Updatable {
  fn update(&mut self);
}

pub trait Generation {
  fn generation(&self) -> u64;
}

