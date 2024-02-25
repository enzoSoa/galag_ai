enum Cell {
    WALL,
    FLOUR,
    START,
    END,
}

impl Cell {
    pub fn from_char(character: char) -> Self {
        match character {
            'X' => Cell::WALL,
            'O' => Cell::FLOUR,
            'S' => Cell::START,
            'E' => Cell::END,
            _ => panic!(
                "
                    Character for a cell must be one of these : \n
                    - 'X' for a wall
                    - 'O' for a floor
                    - 'S' for starting point
                    - 'E' for end point
                "
            ),
        }
    }
}

type Row = Vec<Cell>;

struct Maze {
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
            .map(|character| Cell::from_char(character.clone()))
            .collect();
    }
}

fn main() {
    let _maze = Maze::from_char_pattern(vec![
        vec!['X', 'S', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'O', 'O', 'O', 'O', 'O', 'X'],
        vec!['X', 'O', 'X', 'O', 'O', 'X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'O', 'O', 'X', 'O', 'O', 'X'],
        vec!['X', 'O', 'X', 'X', 'O', 'X', 'X', 'O', 'X'],
        vec!['X', 'O', 'O', 'O', 'O', 'O', 'O', 'O', 'X'],
        vec!['X', 'X', 'X', 'O', 'X', 'O', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'O', 'X', 'O', 'O', 'O', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'X', 'X', 'E', 'X'],
    ]);
}
