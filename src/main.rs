mod board;

use rayon::prelude::*;

static MINUS_INF: i32 = i32::MIN;
static PLUS_INF: i32 = i32::MAX;

fn __main() {
    let mut board = board::new_state();
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::White),
        &board,
        (0, 0),
    );
    board = board::set(
        board::new_piece(board::Type::Pawn, board::Color::White),
        &board,
        (0, 2),
    );
    board = board::set(
        board::new_piece(board::Type::Pawn, board::Color::Black),
        &board,
        (7, 6),
    );
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::Black),
        &board,
        (0, 1),
    );
    let result = alpha_beta(&board, 5, MINUS_INF, PLUS_INF, board::Color::White);
    board::show_state(&board);
    board = board::make_move(&board, (0, 0), (3, 3));
    board::show_state(&board);
    println!("Alpha Beta: {}", result);
    pick_move(&board);
}

fn _main() {
    let mut board = board::new_state();
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::White),
        &board,
        (0, 1),
    );
    let res = board::get_all_moves_for_collor(&board);
    println!("Aval moves: {:?} count: {}", res, res.len());
}

fn main() {
    let mut board = board::new_state();
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::White),
        &board,
        (0, 0),
    );
    board = board::set(
        board::new_piece(board::Type::Pawn, board::Color::White),
        &board,
        (0, 1),
    );
    board = board::set(
        board::new_piece(board::Type::Pawn, board::Color::Black),
        &board,
        (7, 6),
    );
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::Black),
        &board,
        (7, 0),
    );
    let n = 10;
    board::show_state(&board);
    // make n moves
    for _ in 0..n {
        let (_, (from, to)) = pick_move(&board);
        board::show_move(&board, from, to);
        board = board::make_move(&board, from, to);
        //board::show_state(&board);
    }
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

fn pick_move(state: &board::State) -> (i32, (board::Position, board::Position)) {
    let depth = 5;
    let moves = board::get_all_moves_for_collor(state);
    let mut moves_with_scores = vec![];
    let mut i = 0;
    while i < moves.len() {
        moves_with_scores.push((0 as i32, moves[i]));
        i += 1;
    }
    moves_with_scores
        //.par_iter_mut()
        .iter_mut()
        .for_each(|(score, (from, to))| {
            *score = alpha_beta(
                &board::make_move(state, *from, *to),
                depth,
                MINUS_INF,
                PLUS_INF,
                state.color,
            );
        });
    println!("Moves: {:?}", moves_with_scores);
    match state.color {
        board::Color::Black => {
            return *moves_with_scores
                .iter()
                .max_by_key(|(score, _)| {
                    return score;
                })
                .unwrap();
        }
        board::Color::White => {
            return *moves_with_scores
                .iter()
                .min_by_key(|(score, _)| {
                    return score;
                })
                .unwrap();
        }
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
