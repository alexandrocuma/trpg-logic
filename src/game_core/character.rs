use super::{
  stats::Stats, 
  spell_level::SpellLevel, 
  spell_slots::SpellSlots, 
  action_slots::ActionSlots,
  character_classes::CharacterClasses,
};

#[derive(Debug)]
pub struct Character {
  // pub health: usize,
  // pub level: usize,
  // pub name: String,
  pub action_slots: ActionSlots,
  pub spell_slots: SpellSlots,
  pub class: CharacterClasses,
  pub stats: Stats
}

impl Character {
  pub fn new(class: CharacterClasses) -> Character {
    Character {
      stats: Stats::new(&class),
      spell_slots: SpellSlots::new(&class),
      action_slots: ActionSlots::set(&class),
      class,
    }
  }

  pub fn end_turn(&mut self) {
    // dequeue
    self.action_slots = ActionSlots::set(&self.class)
  }

  pub fn attack(&mut self) {
    self.action_slots.use_action()
  }

  pub fn cast_spell(&mut self, level: SpellLevel) {
    if self.action_slots.have_actions() {
      match level {
        SpellLevel::One => {
          self.spell_slots.use_level_one(self.action_slots.use_action())
        },
        _ => println!("Other level spells")
      }
    } else {
      println!("No actions for spells cast")
    }
  }

  fn _die() {

  }
}
