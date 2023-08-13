mod game_core;
mod battle_system;

use game_core::character::Character;
use game_core::spell_level::SpellLevel;
use game_core::character_classes::CharacterClasses;


use battle_system::battle_queue::BattleQueue;

fn main() {
  let barbarian2 = Character::new(CharacterClasses::Barbarian);
  let barbarian = Character::new(CharacterClasses::Barbarian);
  let wizard = Character::new(CharacterClasses::Wizard);

  let characters = vec![
    // barbarian2,
    // barbarian, 
    wizard
  ];

  let mut battle_queue = BattleQueue::set(characters);

  match battle_queue.next_in_queue() {
    Some(mut character) => {
      character.cast_spell(SpellLevel::One);
      character.cast_spell(SpellLevel::Five);
      character.cast_spell(SpellLevel::One);
      character.attack();
      character.attack();
      character.end_turn();
    },
    None => println!("No value"),
  }
}
