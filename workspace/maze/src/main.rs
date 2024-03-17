pub mod agent;
pub mod map;

use map::Map;

use crate::agent::Agent;

fn main() {
    let maze = Map::from_char_pattern(vec![
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
    let agent = Agent::from_coordinates('A', 1, 0);

    print!("{}\n{}\n", agent, maze);
}
