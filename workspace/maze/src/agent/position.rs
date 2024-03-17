use core::fmt;

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
    pub fn from_coordinates(x: u32, y: u32) -> Self {
        Self {
            x: Coordinate(x),
            y: Coordinate(y),
        }
    }

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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[x:{}, y:{}]", self.x, self.y)
    }
}
