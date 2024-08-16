use std::fmt;

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<bool>>, // false represents walls, true represents floor
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,

    visited: Option<Vec<Vec<bool>>>,
    path_map: Option<Vec<Vec<bool>>>,
}

impl Maze {
    // Character for walls: 'X'
    const INPUT_WALL: char = 'X';
    // Character for floor: ' '
    const INPUT_FLOOR: char = ' ';

    const OUTPUT_WALL: char = '⬜';
    const OUTPUT_FLOOR: char = '⬛';
    const OUTPUT_START: char = '❌';
    const OUTPUT_PATH: char = '◽';

    /// Creates a new [`Maze`].

    fn new(map: Vec<Vec<bool>>, start_x: usize, start_y: usize) -> Result<Maze, String> {
        let width = map.iter().map(|row| row.len()).max().unwrap_or_default();

        let height = map.len();

        if height < 3 || width < 3 {
            return Err("Maze is too small. Minimum 3x3".to_string());
        }

        let mut map = map.to_vec();

        // make sure all rows are the same length
        for row in map.iter_mut() {
            row.resize(width, false)
        }

        if let Some(row_vec) = map.get(start_y) {
            if let Some(&value) = row_vec.get(start_x) {
                if !value {
                    return Err("Starting position must not be on a wall!".to_string());
                }
            } else {
                return Err("start_x out of bounds!".to_string());
            }
        } else {
            return Err("start_y out of bounds!".to_string());
        }

        Ok(Self {
            map, // false represents walls, true represents floor
            width,
            height,
            start_x,
            start_y,

            visited: None, //vec![vec![false; width]; height,
            path_map: None,
        })
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for (y, row) in self.map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let symbol = if x == self.start_x && y == self.start_y {
                    Maze::OUTPUT_START
                } else if self
                    .path_map
                    .as_ref()
                    .and_then(|map| map.get(y))
                    .and_then(|row| row.get(x))
                    == Some(&true)
                {
                    Maze::OUTPUT_PATH
                } else {
                    match cell {
                        true => Maze::OUTPUT_FLOOR,
                        false => Maze::OUTPUT_WALL,
                    }
                };
                s.push(symbol);
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

fn main() {
    println!("Hello, world!");

    let original: Vec<Vec<bool>> = vec![
        vec![true, false, true],
        vec![false, true, false],
        vec![true, true, false],
    ];

    let test_maze = Maze::new(original, 1, 1);

    println!("test_maze: \n{}", test_maze.expect("no maze here"));
}
