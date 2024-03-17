use super::position::OffsetVector;

pub enum Direction {
    UP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl Direction {
    pub fn as_position_vector(&self) -> OffsetVector {
        match self {
            Direction::UP => OffsetVector { x: 0, y: -1 },
            Direction::RIGHT => OffsetVector { x: 1, y: 0 },
            Direction::BOTTOM => OffsetVector { x: 0, y: 1 },
            Direction::LEFT => OffsetVector { x: -1, y: 0 },
        }
    }
}
