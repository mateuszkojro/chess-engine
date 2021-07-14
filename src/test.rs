fn test_pawn(){
    let mut board = board::new_state();
    board = board::set(
        board::new_piece(board::Type::Pawn, board::Color::Black),
        board,
        (0, 6),
    );
    board.color = board::Color::Black;
    board::show_state(&board);
    let result = alpha_beta(&board, 1, MINUS_INF, PLUS_INF, board::Color::Black);
}
