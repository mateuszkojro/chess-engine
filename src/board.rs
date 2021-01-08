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
    color: Color,
}

#[allow(dead_code)]
pub enum Move {
    Enemy,
    NotTake,
    Take,
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

/// generujemy pozycje z danego miejsca na pustje szachownicy
fn get_piece_moves(piece: &Piece, s: &State, p: &Position) -> Vec<Position> {
    let all_possible_moves = match piece.rodzaj {
        Type::Pawn => get_pawn_moves(&s, &p),
        Type::Rook => get_rook_moves(&s, &p),
    };
    return get_filterd_moves(all_possible_moves);
}

/// generujemu pozycje z danego miejsca na pustje szachownicy

fn get_rook_moves(s: &State, p: &Position) -> Vec<(Move, Position)> {
    let mut res: Vec<(Move, Position)> = vec![];
    for i in 1..8 {
        res.push((check_rook_position(&s, &p), (p.0, (p.1 + i) % 8)));
        res.push((check_rook_position(&s, &p), ((p.0 + i) % 8, p.1)));
    }
    return res;
}

pub fn get_all_moves_for_collor(s: &State) -> Vec<(Position, Position)> {
    let mut res: Vec<(Position, Position)> = vec![];
    for (position, piece) in s.pieces.iter() {
        if (s.color == piece.color) {
            for to in get_piece_moves(&piece, &s, &position) {
                res.push((*position, to));
            }
        }
    }
    println!("aval moves: {:?}", res);
    return res;
}

/// __nie gotowe koniecznie zmienic__  sprawdzamy czy pole jest puste czy stoi na nim nasz pion czy pion przeciwnika
fn get_pawn_moves(_b: &State, p: &Position) -> Vec<(Move, Position)> {
    // TODO how fast is it?
    // FIXME brakuje podwojnych ruchow i bicia
    return vec![(Move::Take, (p.0, p.1 + 1)), (Move::Take, (p.0, p.1 + 2))];
}

/// sprawdzamy czy pole jest puste czy stoi na nim nasz pion czy pion przeciwnika
fn check_rook_position(s: &State, p: &Position) -> Move {
    match s.pieces.get(&p) {
        Some(piece) => match piece.color == s.color {
            true => Move::NotTake,
            false => Move::Enemy,
        },
        None => Move::Take,
    }
}

/// Majac `Vec<(Move,Position)>` filtrujemy go i zostawiamy tylko pozycje na ktore mozemy przejsc
fn get_filterd_moves(moves: Vec<(Move, Position)>) -> Vec<Position> {
    let mut res: Vec<Position> = vec![];
    for (possible_move, position) in moves {
        match possible_move {
            Move::Take => {
                res.push(position);
            }
            Move::Enemy => {
                res.push(position);
                break;
            }
            Move::NotTake => break,
        }
    }
    return res;
}

/// zwraca obecna ocene szachownicy gdzie czarne pionki maja wartosci ujemne a biale dodatnie
pub fn get_evaluation(s: &State) -> i32 {
    //Maybe done this way s.pieces.into_values().fold()
    let mut res = 0;
    for piece in s.pieces.values() {
        res = res + get_piece_value(piece);
    }
    println!("oceniam na: {}", res);
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
    let mut new_state = s.clone();
    new_state.color = !new_state.color;
    let insert = new_state.pieces.remove(&from).unwrap();
    new_state.pieces.insert(to, insert);
    return new_state;
}

/// Wstawia na podana pozycje `piece`
pub fn set(i: Piece, s: State, to: Position) -> State {
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
                        Type::Pawn => print!("♟︎"),
                        Type::Rook => print!("♜"),
                    },
                    Color::White => match piece.rodzaj {
                        Type::Pawn => print!("♙"),
                        Type::Rook => print!("♖"),
                    },
                },
                None => print!("_"),
            }
        }
        println!("");
    }
}
