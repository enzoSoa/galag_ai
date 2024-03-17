pub mod coordinate;
pub mod direction;
pub mod position;

use core::fmt;

use position::Position;

pub struct Agent {
    pub character: char,
    pub position: Position,
}

impl Agent {
    pub fn from_coordinates(character: char, x: u32, y: u32) -> Self {
        let position = Position::from_coordinates(x, y);
        Self {
            character,
            position,
        }
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.character, self.position)
    }
}
