use ggez::{
    event::{self, EventHandler},
    graphics::{Canvas, Color},
    input::keyboard::{KeyCode, KeyInput},
    Context as GgezContext, GameResult,
};

use crate::{astar::*, bfs::*, dfs::*, maze::*, renderer::*};

mod astar;
mod bfs;
mod dfs;
mod maze;
mod renderer;

const TARGET_FPS: u32 = 30;

enum MazeSolveAlgorithm {
    BFS,
    DFS,
    ASTAR,
}

struct GameState {
    maze: Maze,
    position_start: GridPosition,
    position_end: GridPosition,

    solver_bfs: MazeSolverBFS,
    solver_dfs: MazeSolverDFS,
    solver_astar: MazeSolverAStar,

    current_algorithm: MazeSolveAlgorithm,

    renderer: Renderer,

    path_position_index: usize,
    path_traced: Vec<GridPosition>,
}

impl GameState {
    /// Reset to re-draw start -> finish path
    fn reset_path_rendering(&mut self) {
        self.path_position_index = 0;
        self.path_traced.clear();
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut GgezContext) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS) {
            let solver: &mut dyn MazeSolverDrawable;
            match self.current_algorithm {
                MazeSolveAlgorithm::BFS => solver = &mut self.solver_bfs,
                MazeSolveAlgorithm::DFS => solver = &mut self.solver_dfs,
                MazeSolveAlgorithm::ASTAR => solver = &mut self.solver_astar,
            }

            solver.update();

            // Update path tracing
            if solver.is_complete() && self.path_position_index < solver.path().len() {
                self.path_traced
                    .push(solver.path()[self.path_position_index]);
                self.path_position_index += 1;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut GgezContext) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);

        // Draw base maze
        self.renderer.draw_maze(&mut canvas, &self.maze);

        let solver: &mut dyn MazeSolverDrawable;
        match self.current_algorithm {
            MazeSolveAlgorithm::BFS => solver = &mut self.solver_bfs,
            MazeSolveAlgorithm::DFS => solver = &mut self.solver_dfs,
            MazeSolveAlgorithm::ASTAR => solver = &mut self.solver_astar,
        }

        self.renderer.draw_maze_solver(&mut canvas, solver);

        // Trace path
        for cell in &self.path_traced {
            self.renderer
                .draw_cell(&mut canvas, cell, MazeColor::PATH_TRACE.0);
        }

        canvas.finish(ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut GgezContext,
        input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::B) => {
                self.current_algorithm = MazeSolveAlgorithm::BFS;
                // XXX: Add reset functionality
                self.solver_bfs =
                    MazeSolverBFS::new(self.maze.clone(), self.position_start, self.position_end);
                self.reset_path_rendering();
            }
            Some(KeyCode::D) => {
                self.current_algorithm = MazeSolveAlgorithm::DFS;
                // XXX: Add reset functionality
                self.solver_dfs =
                    MazeSolverDFS::new(self.maze.clone(), self.position_start, self.position_end);
                self.reset_path_rendering();
            }
            Some(KeyCode::A) => {
                self.current_algorithm = MazeSolveAlgorithm::ASTAR;
                // XXX: Add reset functionality
                self.solver_astar =
                    MazeSolverAStar::new(self.maze.clone(), self.position_start, self.position_end);
                self.reset_path_rendering();
            }
            Some(KeyCode::T) => {
                self.reset_path_rendering();
            }
            _ => {}
        }

        Ok(())
    }
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_module(env!("CARGO_PKG_NAME"), log::LevelFilter::Trace)
        .init();

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        log::error!("Usage: maze_solver [maze_file]");
        std::process::exit(1);
    }

    let maze_file_name = args[1].as_str();
    let maze_string = std::fs::read_to_string(maze_file_name).unwrap_or_else(|_| {
        log::error!("Failed to read maze file: {}", maze_file_name);
        std::process::exit(1)
    });

    let position_start = GridPosition::new(1, 3);
    let position_end = GridPosition::new(17, 14);

    let maze = Maze::new_from_string(
        &maze_string
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or(&maze_string),
    )
    .unwrap_or_else(|_| {
        log::error!("Failed to parse maze string: \n{}", maze_string);
        std::process::exit(1)
    });

    let (ctx, events_loop) = ggez::ContextBuilder::new("MazeSolver", "fairywreath")
        .window_setup(ggez::conf::WindowSetup::default().title("Maze Solver"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    let solver_bfs = MazeSolverBFS::new(maze.clone(), position_start, position_end);
    let solver_dfs = MazeSolverDFS::new(maze.clone(), position_start, position_end);
    let solver_astar = MazeSolverAStar::new(maze.clone(), position_start, position_end);

    let renderer = Renderer::new_with_maze(&maze);

    let state = GameState {
        maze,
        position_start,
        position_end,
        solver_bfs,
        solver_dfs,
        solver_astar,
        current_algorithm: MazeSolveAlgorithm::ASTAR,
        renderer,

        path_position_index: 0,
        path_traced: Vec::new(),
    };

    event::run(ctx, events_loop, state)
}
