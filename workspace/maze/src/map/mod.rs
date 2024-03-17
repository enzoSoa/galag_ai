pub mod row;
pub mod tile;

use core::fmt;

use row::Row;

pub struct Map {
    blueprint: Vec<Row>,
}

impl Map {
    pub fn from_char_pattern(char_pattern: Vec<Vec<char>>) -> Self {
        let blueprint: Vec<Row> = char_pattern
            .iter()
            .map(|row| Row::from_char_vector(row))
            .collect();
        return Map {
            blueprint: blueprint,
        };
    }

    pub fn to_string(&self) -> String {
        self.blueprint
            .iter()
            .fold(String::new(), |acc, row| format!("{}\n{}", acc, row))
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
