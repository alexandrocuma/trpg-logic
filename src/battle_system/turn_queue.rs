use crate::game_core::character_class::CharacterClass;

#[derive(Debug)]
pub struct TurnQueue {
  pub characters_in_battle: Vec<CharacterClass>
}

impl TurnQueue {
  pub fn new(mut characters_in_battle: Vec<CharacterClass>) -> Self {
    characters_in_battle.sort_by(|a, b| b.cmp(a));
    TurnQueue {
      characters_in_battle
    }
  }


  // fn sort_queue(&self);

  // fn tied_characters(&self);

  // fn dequeue(&self);

  // fn enqueue(&self);
}
