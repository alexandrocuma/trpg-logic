use crate::{game_core::stats::Stats, characters::{barbarian::Barbarian, wizard::Wizard}};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharacterEnum {
  Barbarian(Character<Barbarian>),
  Wizard(Character<Wizard>),
}

pub trait CharacterType {
}
// pub trait CharacterTrait {
//   // fn move_character(&self);

//   // fn attack(&self);

//   // fn die(&self);

//   // fn read_stats(&self) -> &Stats;
// }

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Character<D: CharacterType>  {
  pub health: usize,
  pub level: usize,
  pub name: String,
  pub stats: Stats,
  pub class: D,
}

impl<T: CharacterType> Character<T> {
  pub fn new(name: String, health: usize, class: T) -> Character<T> {
    Character { 
      name: name, 
      health, 
      level: 1, 
      stats: Stats::new(),
      class: class
    }
  }
}


