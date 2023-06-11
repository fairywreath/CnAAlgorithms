use crate::maze::*;

const STEPS_PER_UPDATE: usize = 5;

pub struct MazeSolverDFS {
    maze: Maze,
    position_start: GridPosition,
    position_end: GridPosition,

    /// True if visited
    search_grid: Vec<Vec<bool>>,

    current_search_positions: Vec<GridPosition>,
    path_edges: MazePathEdges,

    complete: bool,
    complete_path: Vec<GridPosition>,

    expanded_nodes: Vec<GridPosition>,
}

impl MazeSolverDFS {
    pub fn new(maze: Maze, position_start: GridPosition, position_end: GridPosition) -> Self {
        let search_grid = vec![vec![false; maze.grid[0].len()]; maze.grid.len()];
        let current_search_positions = vec![position_start];
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

    /// 1 step of iterative DFS search, expanding the current deepest node by 1
    fn search(&mut self) {
        let mut start = None;
        while !self.current_search_positions.is_empty() {
            let cell = self.current_search_positions.pop().unwrap();
            // Since graph is cyclic, nodes in the search stack may already be visited.
            // Only search from unvisited nodes
            if !self.search_grid[cell.x as usize][cell.y as usize] {
                start = Some(cell);
                break;
            }
        }
        if start.is_none() {
            return;
        }

        let start = start.unwrap();
        self.expanded_nodes.push(start);

        self.search_grid[start.x as usize][start.y as usize] = true;

        for delta in SEARCH_ORDER.iter().rev() {
            let cell = start.add(delta);
            if self.maze.is_cell_valid(&cell) && !self.search_grid[cell.x as usize][cell.y as usize]
            {
                self.current_search_positions.push(cell);
                self.path_edges.add_edge(&start, &cell);

                if cell == self.position_end {
                    self.complete = true;
                    self.complete_path = self
                        .path_edges
                        .build_path(&self.position_start, &self.position_end)
                        .unwrap();
                    self.print_result_path();
                    break;
                }
            }
        }
    }

    // XXX: Do not duplicate this code
    fn print_result_path(&self) {
        log::info!(
            "Pathfinding result for DFS search from ({},{}) to ({},{}) - ",
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

impl MazeSolverDrawable for MazeSolverDFS {
    fn update(&mut self) {
        if !self.complete {
            for _ in 0..STEPS_PER_UPDATE {
                self.search();
            }
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
