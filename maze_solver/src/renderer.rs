use ggez::graphics::{self, Canvas, Color, Rect};

use crate::maze::*;

pub struct Renderer {
    cell_width: f32,
    cell_height: f32,
}

impl Renderer {
    pub fn new_with_maze(maze: &Maze) -> Self {
        let cell_width = SCREEN_WIDTH / maze.grid.len() as f32;
        let cell_height = SCREEN_HEIGHT / maze.grid[0].len() as f32;

        Self {
            cell_width,
            cell_height,
        }
    }

    pub fn draw_maze(&self, canvas: &mut Canvas, maze: &Maze) {
        for (x, row) in maze.grid.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                match cell {
                    0 => {
                        self.draw_cell(
                            canvas,
                            &GridPosition::new(x as _, y as _),
                            MazeColor::EMPTY.0,
                        );
                    }
                    1 => {
                        self.draw_cell(
                            canvas,
                            &GridPosition::new(x as _, y as _),
                            MazeColor::WALL.0,
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn draw_cell(&self, canvas: &mut Canvas, position: &GridPosition, color: Color) {
        // Coordinates are reversed: x - y axis, y - x axis
        let pos_x = (position.y as f32) * self.cell_width;
        let pos_y = (position.x as f32) * self.cell_height;

        let rect = Rect::new(pos_x, pos_y, self.cell_width, self.cell_height);

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new().dest_rect(rect).color(color),
        );
    }

    pub fn draw_maze_solver(&self, canvas: &mut Canvas, solver: &dyn MazeSolverDrawable) {
        for cell in solver.expanded_nodes() {
            self.draw_cell(canvas, &cell, MazeColor::EXPANDED.0);
        }

        self.draw_cell(canvas, solver.start(), MazeColor::START.0);

        self.draw_cell(canvas, solver.end(), MazeColor::END.0);

        if solver.is_complete() {
            for cell in solver.path() {
                self.draw_cell(canvas, &cell, MazeColor::PATH.0);
            }
        }
    }
}
