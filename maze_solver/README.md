# Maze Solver

## Setup
Rust needs to be installed. On Unix you can run:
```
curl https://sh.rustup.rs -sSf | sh
```

For Windows users and more installation information visit https://www.rust-lang.org/tools/install.

## Running
To build and execute the binary, run
```
cargo run maze.txt
```
A relatively large game library is used, hence compilation may take a while.
"maze.txt" is the argument to the maze data file.
<br>
The complete path, path cost and number of explored nodes will be printed in the terminal after the algorithm terminates.



## Controls
The algorithm is default to A*.
Keybindings:
- T - Trace path result
- A - Start new A* algorithm
- B - Start new BFS algorithm
- D - Start new DFS algorithm
