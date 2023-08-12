
use crate::game_core::stats::Stats;

pub trait CharacterTrait {
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Character<D: CharacterTrait>  {
  pub health: usize,
  pub level: usize,
  pub name: String,
  pub stats: Stats,
  pub class: D,
}

impl<T: CharacterTrait> Character<T> {
  pub fn new(name: String, health: usize, stats: Stats, class: T) -> Character<T> {
    Character { 
      name: name, 
      health, 
      level: 1, 
      stats: Stats::new(stats),
      class: class
    }
  }
  
}


