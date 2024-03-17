use core::fmt;

use crate::map::tile::Tile;

pub struct Row(Vec<Tile>);

impl Row {
    pub fn from_char_vector(char_vector: &Vec<char>) -> Self {
        let row: Vec<Tile> = char_vector
            .iter()
            .map(|character| Tile::from_char(character))
            .collect();
        return Row(row);
    }

    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .fold(String::new(), |acc, tile| format!("{acc} {tile}"))
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
