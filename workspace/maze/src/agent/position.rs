use super::coordinate::{Coordinate, OffsetCoordinate};
use super::direction::Direction;

pub struct OffsetVector {
    pub x: OffsetCoordinate,
    pub y: OffsetCoordinate,
}

pub struct Position {
    x: Coordinate,
    y: Coordinate,
}

impl Position {
    fn offset(&self, vector: OffsetVector) -> Self {
        Self {
            x: self.x.offset(vector.x),
            y: self.y.offset(vector.y),
        }
    }

    pub fn moved(&self, direction: Direction) -> Self {
        self.offset(direction.as_position_vector())
    }
}
