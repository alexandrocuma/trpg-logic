use super::character_classes::CharacterClasses;

#[derive(Debug, Default)]

pub struct ActionSlots {
  pub steps: usize,
  pub actions: usize,
  pub additional_actions: usize,
}

impl ActionSlots {
  pub fn set(class: &CharacterClasses) -> Self {
    match class {
      CharacterClasses::Barbarian => ActionSlots {
        steps: 9, 
        actions: 1,
        additional_actions: 0
      },
      CharacterClasses::Wizard => ActionSlots {
        steps: 6, 
        actions: 1,
        additional_actions: 1
      }
    }
  }

  pub fn have_actions(&self) -> bool {
    self.actions > 0
  }

  pub fn use_action(&mut self) {
    match self.actions > 0 {
      true => { 
        self.actions -= 1;
        println!("Action Used")
      },
      false => println!("You dont have actions left")
    }
  }

  pub fn _use_step(&mut self) {
    match self.steps > 0 {
      true => self.steps -= 1,
      false => println!("You dont have steps left")
    }
  }

  pub fn _use_additional_actions(&mut self) {
    match self.additional_actions > 0 {
      true => self.additional_actions -= 1,
      false => println!("You dont have additionals actions left")
    }
  }
}