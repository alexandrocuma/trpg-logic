use super::character_classes::CharacterClasses;

#[derive(Debug, Default)]

pub struct Stats {
  pub dexterity: usize, 
  pub strength: usize, 
  pub intelligence: usize,
  pub constitution: usize,
  pub charisma: usize,
  pub wisdom: usize
}

impl Stats {
  pub fn new(class: &CharacterClasses) -> Stats {
    match class {
      CharacterClasses::Barbarian => Stats {
        dexterity: 4, 
        ..Default::default()
      },
      CharacterClasses::Wizard => Stats {
        dexterity: 2, 
        ..Default::default()
      }
    }
  }
}