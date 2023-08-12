use crate::game_core::character::CharacterEnum;

pub struct TurnQueue {
  pub characters_in_battle: Vec<CharacterEnum>
}

// impl Ord for CharacterEnum {
//   fn cmp(&self, other: &Self) -> Ordering {
//       match (self, other) {
//           (CharacterEnum::Wizard(wiz1), CharacterEnum::Wizard(wiz2)) => wiz1.cmp(wiz2),
//           (CharacterEnum::Barbarian(barb1), CharacterEnum::Barbarian(barb2)) => barb1.cmp(barb2),
//           (CharacterEnum::Wizard(_), CharacterEnum::Barbarian(_)) => Ordering::Less,
//           (CharacterEnum::Barbarian(_), CharacterEnum::Wizard(_)) => Ordering::Greater,
//       }
//   }
// }

impl TurnQueue {
  pub fn new(mut characters_in_battle: Vec<CharacterEnum>) -> Self {
    characters_in_battle.sort();
    TurnQueue {
      characters_in_battle
    }
  }

  // fn sort_queue(&self);

  // fn tied_characters(&self);

  // fn dequeue(&self);

  // fn enqueue(&self);
}
