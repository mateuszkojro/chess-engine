mod board;

static MINUS_INF: i32 = i32::MIN;
static PLUS_INF: i32 = i32::MAX;

fn main() {
    let mut board = board::new_state();
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::White),
        board,
        (0, 0),
    );
    board = board::set(
        board::new_piece(board::Type::Pawn, board::Color::White),
        board,
        (0, 2),
    );
    board = board::set(
        board::new_piece(board::Type::Pawn, board::Color::Black),
        board,
        (7, 6),
    );
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::Black),
        board,
        (0, 1),
    );
    let result = alpha_beta(&board, 2, MINUS_INF, PLUS_INF, board::Color::White);
    board::show_state(&board);
    board = board::make_move(&board,(0,0),(3,3));
    board::show_state(&board);
    println!("Alpha Beta: {}", result);
}

fn _main(){
    let mut board = board::new_state();
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::White),
        board,
        (0, 1),
    );
    let res = board::get_all_moves_for_collor(&board);
    println!("Aval moves: {:?} count: {}", res, res.len());
}


fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn alpha_beta(
    state: &board::State,
    depth: i8,
    mut alpha: i32,
    mut beta: i32,
    max_player: board::Color,
) -> i32 {
    if depth == 0 {
        return board::get_evaluation(&state);
    }

    if max_player == board::Color::White {
        let mut value = MINUS_INF;
        for (from, to) in board::get_all_moves_for_collor(&state) {
            value = max(
                value,
                alpha_beta(
                    &board::make_move(&state, from, to),
                    depth - 1,
                    alpha,
                    beta,
                    board::Color::Black,
                ),
            );
            alpha = max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
        return value;
    } else {
        let mut value = PLUS_INF;
        for (from, to) in board::get_all_moves_for_collor(&state) {
            value = min(
                value,
                alpha_beta(
                    &board::make_move(&state, from, to),
                    depth - 1,
                    alpha,
                    beta,
                    board::Color::White,
                ),
            );
            beta = min(beta, value);
            if beta <= alpha {
                break;
            }
        }
        return value;
    }
}
