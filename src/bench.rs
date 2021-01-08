#![feature(test)]

mod board;
extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_make_move(b: &mut Bencher) {
    let mut board = board::new_state();
    board = board::set(
        board::new_piece(board::Type::Rook, board::Color::White),
        board,
        (0, 0),
    );
    b.iter(|| {
        let i = (0, 0);
        let board = board::make_move(&board, i, (i.0 + 1, i.0 + 1));
    })
}
