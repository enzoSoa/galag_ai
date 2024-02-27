pub mod maze;

use maze::Maze;

fn main() {
    let maze = Maze::from_char_pattern(vec![
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

    print!("{}", maze);
}
