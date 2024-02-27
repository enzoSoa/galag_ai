pub mod row;
pub mod tile;

use core::fmt;

use row::Row;

pub struct Maze {
    blueprint: Vec<Row>,
}

impl Maze {
    pub fn from_char_pattern(char_pattern: Vec<Vec<char>>) -> Self {
        let blueprint: Vec<Row> = char_pattern
            .iter()
            .map(|row| Row::from_char_vector(row))
            .collect();
        return Maze {
            blueprint: blueprint,
        };
    }

    pub fn to_string(&self) -> String {
        self.blueprint
            .iter()
            .fold(String::new(), |acc, row| format!("{}\n{}", acc, row))
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
