extern crate ansi_term;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;


use super::Updatable;
use super::Generation;

pub struct Forest<T: Display + Updatable> {
  pub generation: u64,
  pub width: usize,
  pub height: usize,
  pub trees: Vec<Vec<T>>,
}

impl <T: Display + Clone + Updatable> Forest <T> {
  pub fn new(width: usize, height: usize, tree: T) -> Forest<T> {
    Forest {
      width: width,
      height: height,
      trees: vec!(vec!(tree; width); height),
      generation: 0
    }
  }
}

impl <T: Display + Updatable> Display for Forest<T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    for row in self.trees.iter() {
      for tree in row.iter() {
        try!(write!(f, "{}", tree))
      }
      try!(write!(f, "\n"))
    }
    Ok(())
  }
}

impl <T: Updatable + Display> Updatable for Forest<T> {
  fn update(&mut self) {
    self.generation += 1;
    for row in self.trees.iter_mut() {
      for tree in row.iter_mut() {
        tree.update();
      }
    }
  }
}

impl <T: Updatable + Display> Generation for Forest<T> {
  fn generation(&self) -> u64 {
    self.generation
  }
}

