#[allow(dead_code)]
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Type {
    Pawn,
    Rook,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub rodzaj: Type,
}

pub type Position = (u8, u8);
pub type Board = HashMap<Position, Piece>;

#[derive(Clone)]
pub struct State {
    pub pieces: Board,
    pub color: Color,
}

#[allow(dead_code)]
pub enum Move {
    Take,
    Go,
    Stop,
}

pub fn new_state() -> State {
    return State {
        color: Color::White,
        pieces: HashMap::<Position, Piece>::new(),
    };
}

pub fn new_piece(rodzaj: Type, color: Color) -> Piece {
    return Piece { color, rodzaj };
}

/// w zaleznosci jaki pion stoi na danym miejscu ta funkcje generujemy
fn get_piece_moves(piece: &Piece, s: &State, p: &Position) -> Vec<Position> {
    let all_possible_moves = match piece.rodzaj {
        Type::Pawn => get_pawn_moves(&s, &p),
        Type::Rook => get_rook_moves(&s, &p),
    };
    return all_possible_moves;
}

/// Zwracamy wszystkie ruchy wierzy z danego miejsca
fn get_rook_moves(s: &State, p: &Position) -> Vec<Position> {
    let mut res: Vec<Position> = vec![];

    for i in (p.0 + 1)..8 {
        let new_pos = (i, p.1);
        assert!(new_pos != *p);
        match check_rook_position(s, &new_pos) {
            Move::Take => {
                res.push(new_pos);
                break;
            }
            Move::Go => {
                res.push(new_pos);
            }
            Move::Stop => {
                break;
            }
        }
    }

    for i in 1..(p.0 + 1) {
        let new_pos = ((p.0 - i), p.1);
        assert!(new_pos != *p);
        match check_rook_position(s, &new_pos) {
            Move::Take => {
                res.push(new_pos);
                break;
            }
            Move::Go => {
                res.push(new_pos);
            }
            Move::Stop => {
                break;
            }
        }
    }

    for i in (p.1 + 1)..8 {
        let new_pos = (p.0, i);
        assert!(new_pos != *p);
        match check_rook_position(s, &new_pos) {
            Move::Take => {
                res.push(new_pos);
                break;
            }
            Move::Go => {
                res.push(new_pos);
            }
            Move::Stop => {
                break;
            }
        }
    }

    for i in 1..(p.1 + 1) {
        let new_pos = (p.0, p.1 - i);
        assert!(new_pos != *p);
        match check_rook_position(s, &new_pos) {
            Move::Take => {
                res.push(new_pos);
                break;
            }
            Move::Go => {
                res.push(new_pos);
            }
            Move::Stop => {
                break;
            }
        }
    }

    return res;
}

pub fn get_all_moves_for_collor(s: &State) -> Vec<(Position, Position)> {
    let mut res: Vec<(Position, Position)> = vec![];
    for (position, piece) in s.pieces.iter() {
        // FIXME: tutaj robimy duzo prownan there must be a better way
        if s.color == piece.color {
            for to in get_piece_moves(&piece, &s, &position) {
                res.push((*position, to));
            }
        }
    }
    //println!("aval moves: {:?}", res);
    return res;
}

/// __nie gotowe koniecznie zmienic__  sprawdzamy czy pole jest puste czy stoi na nim nasz pion czy pion przeciwnika
fn get_pawn_moves(b: &State, p: &Position) -> Vec<Position> {
    // TODO how fast is it?
    // FIXME brakuje podwojnych ruchow i bicia
    match b.pieces[p].color {
        Color::Black => match p.1 {
            0 => vec![],
            1 => vec![(p.0, p.1 - 1)],
            _ => vec![(p.0, p.1 - 1), (p.0, p.1 - 2)],
        },
        Color::White => match p.1 {
            7 => vec![],
            6 => vec![(p.0, p.1 + 1)],
            _ => vec![(p.0, p.1 + 1), (p.0, p.1 + 2)],
        },
    }
}

/// sprawdzamy czy pole jest puste czy stoi na nim nasz pion czy pion przeciwnika
fn check_rook_position(s: &State, p: &Position) -> Move {
    match s.pieces.get(&p) {
        Some(piece) => match piece.color == s.color {
            true => Move::Stop,
            false => Move::Take,
        },
        None => Move::Go,
    }
}

/// zwraca obecna ocene szachownicy gdzie czarne pionki maja wartosci ujemne a biale dodatnie
pub fn get_evaluation(s: &State) -> i32 {
    //Maybe done this way s.pieces.into_values().fold()
    let mut res = 0;
    for piece in s.pieces.values() {
        res = res + get_piece_value(piece);
    }
    //println!("oceniam na: {}", res);
    return res;
}

/// Zwraca wartosc danego `Piece`
fn get_piece_value(piece: &Piece) -> i32 {
    match piece.color {
        Color::White => match piece.rodzaj {
            Type::Pawn => 10,
            Type::Rook => 50,
        },
        Color::Black => match piece.rodzaj {
            Type::Pawn => -10,
            Type::Rook => -50,
        },
    }
}

/// Wstawia na podana pozycje `to` element z pozycji `from` jezeli
/// pod `from` nie ma nic __panics__
pub fn make_move(s: &State, from: Position, to: Position) -> State {
    let (x, y) = to;
    if x > 7 || y > 7 {
        assert!(false);
    }
    let mut new_state = s.clone();
    new_state.color = !new_state.color;
    let insert = new_state.pieces.remove(&from).unwrap();
    new_state.pieces.insert(to, insert);
    return new_state;
}

/// Wstawia na podana pozycje `piece`
pub fn set(i: Piece, s: &State, to: Position) -> State {
    let mut new_state = s.clone();
    new_state.pieces.insert(to, i);
    return new_state;
}

/// Pokaz aktualny `state`
pub fn show_state(s: &State) {
    println!("\n## Aktualny color: {:?} ## \n", s.color);
    for y in 0..8 {
        for x in 0..8 {
            print!(" ");
            match s.pieces.get(&(x, y)) {
                Some(piece) => match piece.color {
                    Color::Black => match piece.rodzaj {
                        Type::Pawn => print!("♙"),
                        Type::Rook => print!("♖"),
                    },
                    Color::White => match piece.rodzaj {
                        Type::Pawn => print!("♟︎"),
                        Type::Rook => print!("♜"),
                    },
                },
                None => print!("_"),
            }
        }
        println!("");
    }
}
