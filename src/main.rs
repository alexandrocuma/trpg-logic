mod game_core;
mod battle_system;

use game_core::stats::Stats;
use game_core::character::Character;
use game_core::character_classes::CharacterClasses;

use battle_system::turn_queue::TurnQueue;

fn main() {
  let barbarian2 = Character::new(CharacterClasses::Barbarian, Stats::new(CharacterClasses::Barbarian));
  let barbarian = Character::new(CharacterClasses::Barbarian, Stats { dexterity: 3, ..Default::default() });
  let wizard = Character::new(CharacterClasses::Wizard, Stats::new(CharacterClasses::Wizard));

  let characters = vec![
    barbarian2,
    barbarian, 
    wizard
  ];

  let turn_queue = TurnQueue::set(characters);

  println!("{:?}", turn_queue);
}
