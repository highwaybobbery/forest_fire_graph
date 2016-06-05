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

impl <T: Display + Clone + Updatable + Default> Forest <T> {
  pub fn new(width: usize, height: usize) -> Forest<T> {
    Forest {
      width: width,
      height: height,
      trees: default_trees(width, height),
      generation: 0
    }
  }
}

fn default_trees<T: Default>(width: usize, height: usize) -> Vec<Vec<T>> {
  let mut t_rows = Vec::with_capacity(height);
  for _ in 0..height {
    let mut t_row = Vec::with_capacity(height);
    for _ in 0..width {
      t_row.push(<T>::default());
    }
    t_rows.push(t_row);
  }
  t_rows
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

