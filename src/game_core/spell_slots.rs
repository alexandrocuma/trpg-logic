use super::character_classes::CharacterClasses;

#[derive(Debug, Default)]

pub struct SpellSlots {
  pub level_one: usize, 
  pub level_two: usize, 
  pub level_three: usize,
  pub level_four: usize,
  pub level_five: usize,
  pub level_six: usize
}

impl SpellSlots {
  pub fn new(class: &CharacterClasses) -> SpellSlots {
    match class {
      CharacterClasses::Barbarian => SpellSlots {
        level_one: 1, 
        ..Default::default()
      },
      CharacterClasses::Wizard => SpellSlots {
        ..Default::default()
      }
    }
  }
}