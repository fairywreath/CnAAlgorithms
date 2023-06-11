use std::{cmp::Reverse, collections::BinaryHeap};

use crate::maze::*;

const STEPS_PER_UPDATE: usize = 3;

/// Element type for the open list priority queue
#[derive(Clone, Copy)]
struct SearchNode {
    /// Cost from starting node
    g: f32,

    /// f: g + heuristic
    f: f32,

    position: GridPosition,
}

impl SearchNode {
    fn new(g: f32, f: f32, position: GridPosition) -> Self {
        Self { g, f, position }
    }
}

impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f.partial_cmp(&other.f).unwrap()
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.f.partial_cmp(&other.f)
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

pub struct MazeSolverAStar {
    maze: Maze,
    position_start: GridPosition,
    position_end: GridPosition,

    /// Stores f(n) values
    search_grid: Vec<Vec<f32>>,

    /// Open list of nodes to visit
    current_search_positions: BinaryHeap<Reverse<SearchNode>>,

    path_edges: MazePathEdges,

    complete: bool,
    complete_path: Vec<GridPosition>,

    expanded_nodes: Vec<GridPosition>,
}

impl MazeSolverAStar {
    pub fn new(maze: Maze, position_start: GridPosition, position_end: GridPosition) -> Self {
        let search_grid = vec![vec![f32::MAX; maze.grid[0].len()]; maze.grid.len()];

        let mut current_search_positions = BinaryHeap::new();
        current_search_positions.push(Reverse(SearchNode::new(0.0, 0.0, position_start)));

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

    /// 1 step of iterative A* search, expanding the current deepest node by 1
    fn search(&mut self) {
        if self.current_search_positions.is_empty() {
            return;
        }

        // XXX: Is this true?
        // We might add a node multiple times to the open list, only compute if the f value is the least

        let start = self.current_search_positions.pop().unwrap().0;
        self.expanded_nodes.push(start.position);

        for delta in &SEARCH_ORDER {
            let cell = start.position.add(delta);
            if self.maze.is_cell_valid(&cell) {
                let g = start.g + 1.0;
                let h = self.h(&cell);
                let f = g + h;

                if f < self.search_grid[cell.x as usize][cell.y as usize] {
                    self.path_edges.add_edge(&start.position, &cell);
                    self.search_grid[cell.x as usize][cell.y as usize] = f;
                    self.current_search_positions
                        .push(Reverse(SearchNode::new(g, f, cell)));
                }

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

    /// Heuristic function - distance from end position to position parameter
    fn h(&self, position: &GridPosition) -> f32 {
        // Manhattan distance
        let x = ((position.x - self.position_end.x) as f32).abs();
        let y = ((position.y - self.position_end.y) as f32).abs();
        x + y
    }

    // XXX: Do not duplicate this code
    fn print_result_path(&self) {
        log::info!(
            "Pathfinding result for A* search from ({},{}) to ({},{}) - ",
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

impl MazeSolverDrawable for MazeSolverAStar {
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
