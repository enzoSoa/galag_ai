use core::fmt;

pub type OffsetCoordinate = i32;

pub struct Coordinate(pub u32);

impl Coordinate {
    pub fn offset(&self, offset_coordinate: OffsetCoordinate) -> Self {
        match self.0.checked_add_signed(offset_coordinate) {
            Some(sum) => Self(sum),
            None => Self(self.0),
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
