extern crate ansi_term;

use std::fmt;
use std::fmt::Display;
use self::ansi_term::Colour::*;

#[derive(Clone)]
pub enum Tree {
  Empty,
  Tree,
  Burning,
  Heating,
}

impl fmt::Display for Tree {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      Tree::Empty => Black.paint(" "),
      Tree::Tree => Green.bold().paint("T"),
      Tree::Burning => Red.bold().paint("B"),
      Tree::Heating => Yellow.bold().paint("T"),
    })
  }
}

pub struct Forest<T: Display> {
  pub width: usize,
  pub height: usize,
  pub trees: Vec<Vec<T>>,
}

impl <T: Display + Clone> Forest <T> {
  pub fn new(width: usize, height: usize, tree: T) -> Forest<T> {
    Forest { width: width, height: height, trees: vec!(vec!(tree; width); height) }
  }
}

impl <T: Display> fmt::Display for Forest<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in self.trees.iter() {
      for tree in row.iter() {
        try!(write!(f, "{}", tree))
      }
      try!(write!(f, "\n"))
    }
    Ok(())
  }
}
