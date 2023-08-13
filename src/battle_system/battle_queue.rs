use rand::Rng;
use std::{cmp::Ordering, collections::VecDeque};

use crate::game_core::character::Character;

#[derive(Debug)]
pub struct BattleQueue {
  battle_queue: VecDeque<Character>
}

impl BattleQueue {
  pub fn set(mut characters: Vec<Character>) -> Self {
    let mut battle_queue = VecDeque::new();
    characters.sort_by(|a, b| {
      if b.stats.dexterity == a.stats.dexterity {
        Self::untie_roll()
      } else {
        b.stats.dexterity.cmp(&a.stats.dexterity)
      }
    });
    battle_queue.extend(characters);

    BattleQueue { battle_queue }
  }

  pub fn next_in_queue(&mut self) -> Option<Character> {
    self.battle_queue.pop_front()
  }

  fn untie_roll() -> Ordering {
    let mut rng = rand::thread_rng();

    let a_roll = rng.gen_range(1..=20);
    let b_roll = rng.gen_range(1..=20);

    b_roll.cmp(&a_roll)
  }
}