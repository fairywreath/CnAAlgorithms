const MIN_SUBTRACTION_PER_MOVE: u32 = 1;
const MAX_SUBTRACTION_PER_MOVE: u32 = 3;

/// Result values per recursive round
struct RoundResult {
    /// Minimax score
    best_score: i32,
    /// Amount subtracted from the pile
    num_subtraction: u32,
}

impl RoundResult {
    pub fn new(best_score: i32, num_subtraction: u32) -> Self {
        Self {
            best_score,
            num_subtraction,
        }
    }
}

/// depth: counter variable from max_depth to 0 to track whether max_depth has been reached
fn minimax(state: u32, depth: u32, maximizing_player: bool, max_depth: &u32) -> RoundResult {
    let mut best_move = 0;
    let mut best_score = 0;

    if state == 0 {
        // Current player loses
        if maximizing_player {
            best_score = -1;
        } else {
            best_score = 1;
        }

        println!(
            "[{},{},{},{},{}]",
            state,
            max_depth - depth,
            maximizing_player,
            best_score,
            "N/A at leaf; game over"
        );

        return RoundResult::new(best_score, 0);
    }

    if depth == 0 {
        println!(
            "[{},{},{},{},{}]",
            state,
            max_depth - depth,
            maximizing_player,
            best_score,
            "N/A at leaf; max depth reached"
        );

        return RoundResult::new(best_score, 0);
    }

    if maximizing_player {
        best_score = i32::MIN;
    } else {
        best_score = i32::MAX;
    }

    for num_subtraction in MIN_SUBTRACTION_PER_MOVE..MAX_SUBTRACTION_PER_MOVE + 1 {
        if state >= num_subtraction {
            let current_result = minimax(
                state - num_subtraction,
                depth - 1,
                !maximizing_player,
                max_depth,
            );

            if maximizing_player {
                if current_result.best_score > best_score {
                    best_score = current_result.best_score;
                    best_move = num_subtraction;
                }
            } else {
                if current_result.best_score < best_score {
                    best_score = current_result.best_score;
                    best_move = num_subtraction;
                }
            }
        }
    }

    println!(
        "[{},{},{},{},{}]",
        state,
        max_depth - depth,
        maximizing_player,
        best_score,
        best_move
    );

    return RoundResult::new(best_score, best_move);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 4 {
        println!("Usage: subtraction_game_minimax [number of stones] [depth] [maximizing player]");
        println!("Example: subtraction_game_minimax 9 5 true");
        std::process::exit(1);
    }

    let number_of_stones = args[1].parse::<u32>().unwrap();
    let depth = args[2].parse::<u32>().unwrap();
    let maximizing_player = args[3].parse::<bool>().unwrap();

    println!("Starting subtraction game minimax with parameters: number of stones = {}, depth = {}, maximizing player = {}", number_of_stones, depth, maximizing_player);

    let result = minimax(number_of_stones, depth, maximizing_player, &depth);

    println!("The optimal move is {}", result.num_subtraction);
}
