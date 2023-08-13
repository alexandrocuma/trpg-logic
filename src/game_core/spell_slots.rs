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
        ..Default::default()
      },
      CharacterClasses::Wizard => SpellSlots {
        level_one: 1,
        ..Default::default()
      }
    }
  }

  pub fn use_level_one(&mut self, use_action: ()) {
    match self.level_one > 0 {
      true => { 
        self.level_one -= 1;
        println!("Spell Level 1 used");
        use_action
      },
      false => println!("You dont have level one slots left")
    }
  }
}