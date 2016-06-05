extern crate ansi_term;
extern crate rand;

use self::ansi_term::Colour::*;
use self::rand::Rng;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use super::Updatable;
use super::Generation;

const NEW_TREE_PROB: f32 = 0.005;
const INITIAL_TREE_PROB: f32 = 0.5;
const FIRE_PROB: f32 = 0.001;
#[derive(Clone)]
pub enum Tree {
  Empty,
  Tree,
  Burning,
  Heating,
}

impl Display for Tree {
  fn fmt(&self, f: &mut Formatter) -> Result {
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

fn prob_check(chance: f32) -> bool {
    let roll = rand::thread_rng().gen::<f32>();
    if chance - roll > 0.0 {
        true
    } else {
        false
    }
}
