pub mod tile;

use tile::Tile;

type Row = Vec<Tile>;

pub struct Maze {
    blueprint: Vec<Row>,
}

impl Maze {
    pub fn from_char_pattern(char_pattern: Vec<Vec<char>>) -> Self {
        let blueprint: Vec<Row> = char_pattern
            .iter()
            .map(|row| Maze::get_row_from_char_vector(row.clone()))
            .collect();
        return Maze {
            blueprint: blueprint,
        };
    }

    fn get_row_from_char_vector(char_vector: Vec<char>) -> Row {
        return char_vector
            .iter()
            .map(|character| Tile::from_char(character.clone()))
            .collect();
    }
}
