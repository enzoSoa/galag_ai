pub enum Tile {
    WALL,
    FLOUR,
    START,
    END,
}

impl Tile {
    pub fn from_char(character: char) -> Self {
        match character {
            'X' => Tile::WALL,
            'O' => Tile::FLOUR,
            'S' => Tile::START,
            'E' => Tile::END,
            _ => panic!(
                "
                    Character for a tile must be one of these : \n
                    - 'X' for a wall
                    - 'O' for a floor
                    - 'S' for starting point
                    - 'E' for end point
                "
            ),
        }
    }
}
