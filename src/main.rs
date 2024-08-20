mod maze {
    use std::fmt;

    pub trait AsChar {
        fn as_char(&self) -> char;
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub enum MazeCell {
        #[default]
        Wall,
        Floor(FloorType),
    }

    impl AsChar for MazeCell {
        fn as_char(&self) -> char {
            match self {
                MazeCell::Wall => '⬜',
                MazeCell::Floor(f) => f.as_char(),
            }
        }
    }

    impl MazeCell {
        fn from_bool(value: bool) -> Self {
            if value {
                MazeCell::Floor(FloorType::default())
            } else {
                MazeCell::default()
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub enum FloorType {
        #[default]
        Floor,
        Start,
        Path,
    }

    impl AsChar for FloorType {
        fn as_char(&self) -> char {
            match self {
                FloorType::Floor => '⬛',
                FloorType::Start => '❌',
                FloorType::Path => '👣',
            }
        }
    }

    #[derive(Debug)]
    pub struct Maze {
        /// `false` represents walls, `true` represents floor
        map: Vec<Vec<MazeCell>>, // false represents walls, true represents floor
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,

        visited: Option<Vec<Vec<bool>>>,
    }

    pub trait Solvable {
        fn solve(&mut self) -> Result<bool, String>;
    }

    impl Maze {
        /// Character for walls: 'X'
        const INPUT_WALL: char = 'X';
        /// Character for floor: ' '
        const INPUT_FLOOR: char = ' ';

        /// Creates a new [`Maze`].
        pub fn new(
            mut map: Vec<Vec<MazeCell>>,
            start_x: usize,
            start_y: usize,
        ) -> Result<Maze, String> {
            let height = map.len();
            let width = map.iter().map(|row| row.len()).max().unwrap_or_default();

            if height < 3 || width < 3 {
                return Err("Maze is too small. Minimum 3x3".to_string());
            }

            // make sure all rows are the same length
            for row in map.iter_mut() {
                // fill shorter rows with default MazeCell (Wall)
                row.resize(width, MazeCell::default())
            }

            Self::validate_start(&map, start_x, start_y)?;

            Ok(Self {
                map,
                width,
                height,
                start_x,
                start_y,
                visited: None,
            })
        }

        pub fn new_from_bool_array(
            map: Vec<Vec<bool>>,
            start_x: usize,
            start_y: usize,
        ) -> Result<Maze, String> {
            let new_map = map
                .into_iter()
                .map(|row| row.into_iter().map(MazeCell::from_bool).collect())
                .collect();
            Maze::new(new_map, start_x, start_y)
        }

        pub fn new_from_str_array(
            map: Vec<&str>,
            start_x: usize,
            start_y: usize,
        ) -> Result<Maze, String> {
            let grid: Result<Vec<Vec<MazeCell>>, String> = map
                .iter()
                .map(|&row| {
                    row.chars()
                        .map(|c| match c {
                            Maze::INPUT_FLOOR => Ok(MazeCell::Floor(FloorType::default())),
                            Maze::INPUT_WALL => Ok(MazeCell::Wall),
                            _ => Err(format!("Unknown character '{}' in provided maze data!", c)),
                        })
                        .collect::<Result<Vec<MazeCell>, String>>()
                })
                .collect();

            // propagate any errors
            let grid = grid?;

            Maze::new(grid, start_x, start_y)
        }

        pub fn new_from_str(map: &str, start_x: usize, start_y: usize) -> Result<Maze, String> {
            let array_map = map.split('\n').collect::<Vec<&str>>();
            Maze::new_from_str_array(array_map, start_x, start_y)
        }

        fn solve_from(&mut self, x: usize, y: usize) -> Result<bool, String> {
            if let (Some(cell), Some(visited)) = (
                self.map.get_mut(y).and_then(|row| row.get_mut(x)),
                self.visited
                    .as_mut()
                    .and_then(|v| v.get_mut(y).and_then(|row| row.get_mut(x))),
            ) {
                if cell == &MazeCell::Wall {
                    // on wall
                    return Ok(false);
                }

                if *visited {
                    // already visited
                    return Ok(false);
                }

                *visited = true;

                if x == 0 || x >= self.width - 1 || y == 0 || y >= self.height - 1 {
                    // found edge (finish)
                    *cell = MazeCell::Floor(FloorType::Path);
                    return Ok(true);
                }

                // Try to solve from neighboring positions
                for (next_x, next_y) in [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ] {
                    if self.solve_from(next_x, next_y)? {
                        //*cell = MazeCell::Floor(FloorType::Path);  // NOTE: here not possible because of borrow checker

                        if let Some(cell) = self.map.get_mut(y).and_then(|row| row.get_mut(x)) {
                            *cell = MazeCell::Floor(FloorType::Path);
                            return Ok(true);
                        } else {
                            return Err(format!("Starting position ({}, {}) out of bounds", x, y));
                        }
                    }
                }
                Ok(false)
            } else {
                Err(format!("Starting position ({}, {}) out of bounds", x, y))
            }
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.height
        }

        pub fn start_x(&self) -> usize {
            self.start_x
        }

        pub fn start_y(&self) -> usize {
            self.start_y
        }

        /// Sets the start of this [`Maze`].
        ///
        /// # Errors
        ///
        /// This function will return an error if starting position is on a wall.
        pub fn set_start(mut self, start_x: usize, start_y: usize) -> Result<Self, String> {
            Self::validate_start(&self.map, start_x, start_y)?;
            self.start_x = start_x;
            self.start_y = start_y;
            Ok(self)
        }

        fn validate_start(
            map: &[Vec<MazeCell>],
            start_x: usize,
            start_y: usize,
        ) -> Result<(), String> {
            if let Some(row_vec) = map.get(start_y) {
                if let Some(value) = row_vec.get(start_x) {
                    match &value {
                        MazeCell::Wall => {
                            Err("Starting position must not be on a wall!".to_string())
                        }
                        MazeCell::Floor(_) => Ok(()),
                    }
                } else {
                    Err("start_x out of bounds!".to_string())
                }
            } else {
                Err("start_y out of bounds!".to_string())
            }
        }
    }

    impl fmt::Display for Maze {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut s = String::new();

            for (y, row) in self.map.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if x == self.start_x && y == self.start_y {
                        // start position
                        s.push(FloorType::Start.as_char())
                    } else {
                        s.push(cell.as_char())
                    }
                }
                s.push('\n');
            }

            write!(f, "{}", s)
        }
    }

    impl Solvable for Maze {
        fn solve(&mut self) -> Result<bool, String> {
            self.visited = Some(vec![vec![false; self.width]; self.height]);
            self.solve_from(self.start_x, self.start_y)
        }
    }
}

fn main() {
    use maze::Maze;
    use maze::Solvable;

    let mut mazes = Vec::new();

    mazes.push(
        Maze::new_from_bool_array(
            vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, true, false],
            ],
            1,
            1,
        )
        .expect("Error while creating maze!"),
    );

    mazes.push(
        Maze::new_from_str_array(vec![" X ", "X X", "  X"], 1, 1)
            .expect("Error while creating maze!")
            .set_start(1, 2)
            .expect("Error on update starting position"),
    );

    mazes.push(
        Maze::new_from_str(
            &("XXX  XX   \n".to_owned()
                + "X     X  X\n"
                + "X XX  XX X\n"
                + "X   XXX   \n"
                + "X    X  XX\n"
                + "XX  XX  X \n"
                + "X  X  X X \n"
                + "X   X   XX\n"
                + "X XXXX XXX\n"
                + "XX  XX  XX"),
            4,
            4,
        )
        .expect("Error while creating maze!"),
    );

    mazes.push(
        Maze::new_from_str(
            &("XXXXXXXXX\n".to_owned()
                + "XXXXXXXXX\n"
                + "XXXXXXXXX\n"
                + "XXX   XXX\n"
                + "XXX   XXX\n"
                + "XXX   XXX\n"
                + "XXX   XXX\n"
                + "XXXXXXXXX\n"
                + "XXXXXXXXX\n"
                + "XXXXXXXXX"),
            4,
            4,
        )
        .expect("Error while creating maze!"),
    );

    for mut maze in mazes {
        println!(
            "Maze: width: {}, height: {}, start: (x:{}|y:{})\n{}",
            maze.width(),
            maze.height(),
            maze.start_x(),
            maze.start_y(),
            maze
        );

        if let Ok(true) = maze.solve() {
            println!("Solution:\n{}", maze);
        } else {
            println!("No solution for this maze");
        }
    }
}
