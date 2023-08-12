#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Stats {
  pub dexterity: usize, 
  pub strength: usize, 
  pub intelligence: usize,
  pub constitution: usize,
  pub charisma: usize,
  pub wisdom: usize
}

impl Stats {
  pub fn new(data: Stats) -> Self {
    Stats { 
      charisma: 1 + data.charisma, 
      constitution: 1 + data.constitution,  
      dexterity: 1 + data.dexterity, 
      intelligence: 1 + data.intelligence, 
      strength: 1 + data.strength, 
      wisdom: 1 + data.wisdom,
    }
  }
}

