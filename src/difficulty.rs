pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Difficulty {

    pub fn from_string(string: String) -> Option<Difficulty> {
        match string.as_slice() {
            "easy" => Some(Difficulty::Easy),
            "medium" => Some(Difficulty::Medium),
            "hard" => Some(Difficulty::Hard),
            _ => None
        }
    }

    pub fn num_to_remove(&self) -> usize {
        match self {
            &Difficulty::Easy => 45,
            &Difficulty::Medium => 60,
            &Difficulty::Hard => 72
        }
    }

}
