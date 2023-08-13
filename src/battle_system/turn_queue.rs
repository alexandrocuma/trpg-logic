use crate::game_core::character::Character;

#[derive(Debug)]
pub struct TurnQueue {
  
}

impl TurnQueue {
  pub fn set(mut battle_queue: Vec<Character>) -> Vec<Character> {
    battle_queue.sort_by(|a, b| b.stats.dexterity.cmp(&a.stats.dexterity));

    battle_queue
  }
}