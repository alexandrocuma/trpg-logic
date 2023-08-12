mod battle_system;
mod characters;
mod game_core;

use game_core::character::Character;
use game_core::character_class::CharacterClass;

use battle_system::turn_queue::TurnQueue;

use characters::barbarian::Barbarian;
use characters::wizard::Wizard;
use game_core::stats::Stats;

fn main() {
  let barbarian = Character::new(String::from("theBarb"), 12, Stats { dexterity: 3, ..Default::default() }, Barbarian {});
  let wizard: Character<Wizard> = Character::new(String::from("theWiza"), 8, Stats { dexterity: 1, ..Default::default() }, Wizard {});
  let wizard2: Character<Wizard> = Character::new(String::from("theWiza2"), 6, Stats { dexterity: 5, ..Default::default() }, Wizard {});
  let barbarian2 = Character::new(String::from("theBarb"), 12, Stats { dexterity: 4, ..Default::default() }, Barbarian {});

  let characters: Vec<CharacterClass> = vec![
    CharacterClass::Wizard(wizard),
    CharacterClass::Wizard(wizard2),
    CharacterClass::Barbarian(barbarian),
    CharacterClass::Barbarian(barbarian2),
  ];

  let turn_queue = TurnQueue::new(characters);

  println!("{:?}", turn_queue);
}
