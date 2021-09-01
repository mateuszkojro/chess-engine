#![allow(unused)]

#[derive(Copy, Clone, Hash, Debug, PartialEq)]
enum Figure {
    WhitePawn,
    WhiteRook,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKing,
}

/// true - white, false - black
type Color = bool;

// TODO: This type could be changed but idk which is better
// #[derive(Copy, Clone, Hash, Debug)]
// enum Color {
//     Black = -1,
//     White = 1,
// }

// impl std::ops::Not for Color {
//     fn not(&self) -> Color {
//         Color {-(*self as isize)}
//     }
// }

type MoveList = Vec<State>;

fn pawn_moves(state: &State, position: Vec2) -> MoveList {
    let result: MoveList = vec![];
    println!("No moves for pawns");
    result
}
fn rook_moves(state: &State, position: Vec2) -> MoveList {
    let result: MoveList = vec![];
    let mut new_x = position.x + 1;
    while new_x < 8 {
        let new_position = Vec2::new(new_x, position.y);
        assert_ne!(new_position, position);

        match state.get(new_position) {
            Some(piece) => {}
            None => {
                new_x += 1;
            }
        }
    }
    result
}
fn king_moves(state: &State, position: Vec2) -> MoveList {
    let p = position;
    let range = (0..=7);

    let mut possible_x = vec![p.x];
    if range.contains(&(p.x + 1)) {
        possible_x.push(p.x + 1);
    }
    if range.contains(&(p.x - 1)) {
        possible_x.push(p.x - 1);
    }

    let mut possible_y = vec![p.y];
    if range.contains(&(p.y + 1)) {
        possible_y.push(p.y + 1);
    }
    if range.contains(&(p.y - 1)) {
        possible_y.push(p.y - 1);
    }
    let mut result = vec![];

    for x in possible_x {
        for y in &possible_y {
            if x != p.x && *y != p.y {
                result.push(state.make_move(position, Vec2::new(x, *y)))
            }
        }
    }

    result
}

impl Figure {
    fn value(&self) -> i32 {
        match self {
            Figure::WhitePawn => 1,
            Figure::WhiteRook => 10,
            Figure::WhiteKing => 1000,
            Figure::BlackPawn => -1,
            Figure::BlackRook => -10,
            Figure::BlackKing => -1000,
        }
    }

    // Should this be fastly copiable and bigger or smaller? who knows need to tes
    fn moves(&self, current_state: State, position: Vec2) -> MoveList {
        match self {
            Figure::WhitePawn => pawn_moves(&current_state, position),
            Figure::BlackPawn => pawn_moves(&current_state, position),
            Figure::WhiteRook => rook_moves(&current_state, position),
            Figure::BlackRook => rook_moves(&current_state, position),
            Figure::WhiteKing => king_moves(&current_state, position),
            Figure::BlackKing => king_moves(&current_state, position),
        }
    }
}

#[derive(Copy, Clone, Debug)]
// We could store addr and then we would calculate it only once
struct Vec2 {
    x: i32,
    y: i32,
}

// Should we cache calculated adress?
impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        assert!(x < 8);
        assert!(x >= 0);
        assert!(y < 8);
        assert!(y >= 0);
        Vec2 { x, y }
    }
}

#[derive(Copy, Clone, Hash, Debug)]
struct State {
    active_color_: Color,
    board_: [Option<Figure>; 64],
}
fn translate(position: Vec2) -> usize {
    8_usize * position.y as usize + position.x as usize
}
impl State {
    fn empty() -> State {
        State {
            board_: [None; 64],
            active_color_: true,
        }
    }

    fn set(&mut self, position: Vec2, figure: Option<Figure>) {
        //TODO: we should check in hash if evaluation was calculated
        self.board_[translate(position)] = figure;
    }

    fn make_move(&self, from: Vec2, to: Vec2) -> State {
        let mut new_state = *self;
        new_state.active_color_ = !self.active_color_;
        let fig = self.get(from);
        assert_ne!(fig, None);
        new_state.set(to, fig);
        new_state.set(from, None);
        new_state
    }

    // Not rly sure about that
    fn evaluate(&self) -> i32 {
        self.board_.iter().flatten().map(|x| x.value()).sum::<i32>()
    }

    fn get(&self, position: Vec2) -> Option<Figure> {
        self.board_[translate(position)]
    }
}

fn main() {
    let mut board = State::empty();
    board.set(Vec2::new(1, 1), Some(Figure::WhiteKing));
    println!("{:?}", board.evaluate())
}
