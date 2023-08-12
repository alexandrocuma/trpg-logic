use std::cmp::Ordering;

use crate::Character;
use crate::characters::{barbarian::Barbarian, wizard::Wizard};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum CharacterClass {
  Barbarian(Character<Barbarian>),
  Wizard(Character<Wizard>),
}

impl CharacterClass {
  fn get_character_dexterity(&self) -> usize {
    match &self {
      CharacterClass::Wizard(wizard) => wizard.stats.dexterity,
      CharacterClass::Barbarian(barbarian) => barbarian.stats.dexterity,
    }
  }
}

impl Ord for CharacterClass {
  fn cmp(&self, other: &Self) -> Ordering {
    let self_stats = &self.get_character_dexterity();
    let other_stats = other.get_character_dexterity();
    
    self_stats.cmp(&other_stats)
  }
}