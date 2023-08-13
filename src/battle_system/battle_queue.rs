use rand::Rng;
use std::cmp::Ordering;

use crate::game_core::character::Character;

#[derive(Debug)]
pub struct BattleQueue {
  
}

impl BattleQueue {
  pub fn set(mut battle_queue: Vec<Character>) -> Vec<Character> {
    battle_queue.sort_by(|a, b| {
      if b.stats.dexterity == a.stats.dexterity {
        Self::untie_roll()
      } else {
        b.stats.dexterity.cmp(&a.stats.dexterity)
      }
    });

    battle_queue
  }

  fn untie_roll() -> Ordering {
    let mut rng = rand::thread_rng();

    let a_roll = rng.gen_range(1..=20);
    let b_roll = rng.gen_range(1..=20);

    b_roll.cmp(&a_roll)
  }
}