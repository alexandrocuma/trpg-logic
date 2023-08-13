mod game_core;
mod battle_system;

use game_core::character::Character;
use game_core::character_classes::CharacterClasses;

use battle_system::battle_queue::BattleQueue;

fn main() {
  let barbarian2 = Character::new(CharacterClasses::Barbarian);
  let barbarian = Character::new(CharacterClasses::Barbarian);
  let wizard = Character::new(CharacterClasses::Wizard);

  let characters = vec![
    barbarian2,
    barbarian, 
    wizard
  ];

  let battle_queue = BattleQueue::set(characters);

  println!("{:?}", battle_queue);

  for next_battler in battle_queue {
    println!("{:?}", next_battler);
  }
}
