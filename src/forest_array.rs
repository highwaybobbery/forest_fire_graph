extern crate ansi_term;
extern crate rand;

use std::fmt;
use std::fmt::Display;
use self::ansi_term::Colour::*;
use self::rand::Rng;

const NEW_TREE_PROB: f32 = 0.01;
const INITIAL_TREE_PROB: f32 = 0.5;
const FIRE_PROB: f32 = 0.001;

pub trait Updatable {
  fn update(&mut self);
}

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

impl Updatable for Tree {
  fn update(&mut self) {
    *self = match *self {
        Tree::Empty => {
            if prob_check(NEW_TREE_PROB) == true {
                Tree::Tree
            } else {
                Tree::Empty
            }
        }
        Tree::Tree => {
            if prob_check(FIRE_PROB) == true {
                Tree::Burning
            } else {
                Tree::Tree
            }
        }
        Tree::Burning => Tree::Empty,
        Tree::Heating => Tree::Burning,
    }
  }
}

pub struct Forest<T: Display + Updatable> {
  pub width: usize,
  pub height: usize,
  pub trees: Vec<Vec<T>>,
}

impl <T: Display + Clone + Updatable> Forest <T> {
  pub fn new(width: usize, height: usize, tree: T) -> Forest<T> {
    Forest { width: width, height: height, trees: vec!(vec!(tree; width); height) }
  }
}

impl <T: Display + Updatable> fmt::Display for Forest<T> {
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

impl <T: Updatable + Display> Updatable for Forest<T> {
  fn update(&mut self) {
    for row in self.trees.iter_mut() {
      for tree in row.iter_mut() {
        tree.update();
      }
    }
  }
}

fn prob_check(chance: f32) -> bool {
    let roll = rand::thread_rng().gen::<f32>();
    if chance - roll > 0.0 {
        true
    } else {
        false
    }
}
