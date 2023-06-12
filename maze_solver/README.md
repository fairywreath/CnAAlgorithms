# Maze Solver

## Setup
Rust needs to be installed. On Unix you can run:
```
curl https://sh.rustup.rs -sSf | sh
```

For Windows users and more installation information visit https://www.rust-lang.org/tools/install.

## Running
Run 
```
cargo run maze.txt
```
to build and execute the binary. A relatively large game library is used, hence compilation may take a while.
"maze.txt" is the argument to the maze data file.

## Controls
The algorithm is default to A*.
- T - Trace path result
- A - Start new A* algorithm
- B - Start new BFS algorithm
- D - Start new DFS algorithm
