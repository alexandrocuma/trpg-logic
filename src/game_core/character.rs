use super::{character_classes::CharacterClasses, stats::Stats};

#[derive(Debug)]
pub struct Character {
  // pub health: usize,
  // pub level: usize,
  // pub name: String,
  pub class: CharacterClasses,
  pub stats: Stats
}

impl Character {
  pub fn new(class: CharacterClasses, stats: Stats) -> Character {
    Character { class, stats }
  }
}
