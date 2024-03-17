pub type OffsetCoordinate = i32;

pub struct Coordinate(u32);

impl Coordinate {
    pub fn offset(&self, offset_coordinate: OffsetCoordinate) -> Self {
        match self.0.checked_add_signed(offset_coordinate) {
            Some(sum) => Self(sum),
            None => Self(self.0),
        }
    }
}
