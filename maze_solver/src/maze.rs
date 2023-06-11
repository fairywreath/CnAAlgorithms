use anyhow::Result;
use ggez::graphics::Color;

pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 1200.0;

pub const SEARCH_ORDER: [GridPosition; 4] = [
    // Up
    GridPosition { x: -1, y: 0 },
    // Right
    GridPosition { x: 0, y: 1 },
    // Down
    GridPosition { x: 1, y: 0 },
    // Left
    GridPosition { x: 0, y: -1 },
];

pub struct MazeColor(pub Color);

impl MazeColor {
    pub const EMPTY: Self = Self(Color::WHITE);
    pub const WALL: Self = Self(Color::BLACK);
    pub const START: Self = Self(Color::GREEN);
    pub const END: Self = Self(Color::RED);
    pub const EXPANDED: Self = Self(Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 0.5,
    });
    // pub const TO_BE_EXPANDED: Self = Self(Color::CYAN);
    pub const PATH: Self = Self(Color::MAGENTA);
    pub const PATH_TRACE: Self = Self(Color::RED);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition { x, y }
    }

    pub fn add(self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Clone)]
pub struct Maze {
    pub grid: Vec<Vec<u8>>,
}

impl Maze {
    pub fn new_from_string(input: &str) -> Result<Maze> {
        let grid = input
            .trim()
            .split('\n')
            .map(|row| {
                row.trim()
                    .strip_prefix('[')
                    .and_then(|row| row.strip_suffix("],"))
                    // Manually remove leading '[' and ending ']'
                    .unwrap_or_else(|| &row[2..row.len() - 1])
                    .split(',')
                    .map(|elem| elem.trim().parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        log::info!("Parsed maze grid:");
        grid.iter().for_each(|row| {
            log::info!("{:?}", row);
        });

        Ok(Maze { grid })
    }

    /// Whether cell position is valid and not a wall
    pub fn is_cell_valid(&self, position: &GridPosition) -> bool {
        position.x >= 0
            && position.x < self.grid.len() as _
            && position.y >= 0
            && position.y < self.grid[0].len() as _
            && self.grid[position.x as usize][position.y as usize] == 0
    }
}

const INVALID_POSITION: GridPosition = GridPosition { x: -1, y: -1 };

/// Directed 2D grid "graph" for a path
pub struct MazePathEdges {
    /// Adjacencu matrix to store source nodes
    pub edges: Vec<Vec<GridPosition>>,
}

impl MazePathEdges {
    pub fn new_with_maze(maze: &Maze) -> Self {
        // Mark positions as INVALID_POSITION to signify that it is unvisited
        let edges = vec![vec![INVALID_POSITION; maze.grid[0].len()]; maze.grid.len()];
        Self { edges }
    }

    pub fn add_edge(&mut self, source: &GridPosition, dest: &GridPosition) {
        self.edges[dest.x as usize][dest.y as usize] = source.clone();
    }

    pub fn build_path(
        &self,
        start: &GridPosition,
        end: &GridPosition,
    ) -> Result<Vec<GridPosition>> {
        let mut path = Vec::new();
        path.push(end.clone());

        let mut current_dest = end.clone();
        let mut current_source = self.edges[current_dest.x as usize][current_dest.y as usize];

        while current_source != INVALID_POSITION && &current_dest != start {
            path.push(current_source);

            current_dest = current_source;
            current_source = self.edges[current_dest.x as usize][current_dest.y as usize];
        }

        if &current_dest != start {
            return Err(anyhow::anyhow!("Failed to build path!"));
        }

        path.reverse();
        Ok(path)
    }
}

pub trait MazeSolverDrawable {
    fn update(&mut self);

    fn expanded_nodes(&self) -> &Vec<GridPosition>;
    fn start(&self) -> &GridPosition;
    fn end(&self) -> &GridPosition;

    fn is_complete(&self) -> bool;
    fn path(&self) -> &Vec<GridPosition>;
}
