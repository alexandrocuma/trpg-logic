mod battle_system;
mod characters;
mod game_core;

use game_core::character::Character;
use game_core::character::CharacterEnum;

use battle_system::turn_queue::TurnQueue;

use characters::barbarian::Barbarian;
use characters::wizard::Wizard;

fn main() {
  let barbarian = Character::new(String::from("theBarb"), 12, Barbarian {});
  let wizard: Character<Wizard> = Character::new(String::from("theWiza"), 8, Wizard {});
  let wizard2: Character<Wizard> = Character::new(String::from("theWiza2"), 6, Wizard {});

  let mut characters: Vec<CharacterEnum> = vec![
    CharacterEnum::Wizard(wizard),
    CharacterEnum::Wizard(wizard2),
    CharacterEnum::Barbarian(barbarian),
  ];

  let turn_queue = TurnQueue::new(characters);
  for character in turn_queue.characters_in_battle {
    match character {
      CharacterEnum::Wizard(wizard) => {
        println!("{:?}", wizard);
      }
      CharacterEnum::Barbarian(barbarian) => {
        println!("{:?}", barbarian);
      }
    }
  }
}
