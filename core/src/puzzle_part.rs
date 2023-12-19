#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PuzzlePart {
    One,
    Two,
}

impl PuzzlePart {
    pub fn new(input: u8) -> Self {
        match input {
            1 => PuzzlePart::One,
            2 => PuzzlePart::Two,
            _ => panic!("part can only be 1 or 2."),
        }
    }
}
