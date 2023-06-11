use std::collections::VecDeque;

use crate::maze::*;

pub struct MazeSolverBFS {
    maze: Maze,
    position_start: GridPosition,
    position_end: GridPosition,

    /// 0 - new block, 1 - to be visited, 2 - visited
    search_grid: Vec<Vec<u8>>,

    current_search_positions: VecDeque<GridPosition>,
    path_edges: MazePathEdges,

    complete: bool,
    complete_path: Vec<GridPosition>,

    expanded_nodes: Vec<GridPosition>,
}

impl MazeSolverBFS {
    pub fn new(maze: Maze, position_start: GridPosition, position_end: GridPosition) -> Self {
        let search_grid = vec![vec![0; maze.grid[0].len()]; maze.grid.len()];

        let mut current_search_positions = VecDeque::new();
        current_search_positions.push_back(position_start);

        let path_edges = MazePathEdges::new_with_maze(&maze);

        Self {
            maze,
            position_start,
            position_end,
            search_grid,

            current_search_positions,
            path_edges,

            complete: false,
            complete_path: Vec::new(),

            expanded_nodes: Vec::new(),
        }
    }

    /// 1 step of BFS search, expanding all hinge nodes by 1
    fn search(&mut self) {
        // Number of expansions in current search/update step
        let num_expansions = self.current_search_positions.len();

        for _ in 0..num_expansions {
            let start = self.current_search_positions.pop_front().unwrap();
            self.expanded_nodes.push(start);

            for delta in &SEARCH_ORDER {
                let cell = start.add(delta);
                if self.maze.is_cell_valid(&cell)
                    && self.search_grid[cell.x as usize][cell.y as usize] == 0
                {
                    // Mark as "to be visited"
                    self.search_grid[cell.x as usize][cell.y as usize] = 1;
                    self.current_search_positions.push_back(cell);
                    self.path_edges.add_edge(&start, &cell);

                    if cell == self.position_end {
                        self.complete = true;
                        self.complete_path = self
                            .path_edges
                            .build_path(&self.position_start, &self.position_end)
                            .unwrap();
                        self.print_result_path();
                    }
                }
            }
            self.search_grid[start.x as usize][start.y as usize] = 2;

            if self.complete {
                break;
            }
        }
    }

    // XXX: Do not duplicate this code
    fn print_result_path(&self) {
        log::info!(
            "Pathfinding result for BFS search from ({},{}) to ({},{}) - ",
            self.position_start.x,
            self.position_start.y,
            self.position_end.x,
            self.position_end.y
        );

        let mut path_positions = String::from("Complete path: ");
        for position in &self.complete_path {
            path_positions += format!(" ({}, {})", position.x, position.y).as_str();
        }
        log::info!("{}", path_positions);

        log::info!("Path cost: {}", self.complete_path.len());
        log::info!("Number of explored nodes: {}", self.expanded_nodes.len());
    }
}

impl MazeSolverDrawable for MazeSolverBFS {
    fn update(&mut self) {
        if !self.complete {
            self.search();
        }
    }

    fn expanded_nodes(&self) -> &Vec<GridPosition> {
        &self.expanded_nodes
    }

    fn start(&self) -> &GridPosition {
        &self.position_start
    }

    fn end(&self) -> &GridPosition {
        &self.position_end
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn path(&self) -> &Vec<GridPosition> {
        &self.complete_path
    }
}
