use super::{character_classes::CharacterClasses, stats::Stats, spell_slots::SpellSlots};

#[derive(Debug)]
pub struct Character {
  // pub health: usize,
  // pub level: usize,
  // pub name: String,
  pub spell_slots: SpellSlots,
  pub class: CharacterClasses,
  pub stats: Stats
}

impl Character {
  pub fn new(class: CharacterClasses) -> Character {
    Character {
      stats: Stats::new(&class),
      spell_slots: SpellSlots::new(&class),
      class,
    }
  }
}
