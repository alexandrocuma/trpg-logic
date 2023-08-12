#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stats {
  dexterity: usize,
  strength: usize, 
  intelligence: usize,
  constitution: usize,
  charisma: usize,
  wisdom: usize
}

impl Stats {
  pub fn new() -> Self {
    Stats { 
      charisma: 1, 
      constitution: 1, 
      dexterity: 1, 
      intelligence: 1, 
      strength: 1, 
      wisdom: 1,
    }
  }
}

