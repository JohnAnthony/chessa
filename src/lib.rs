use std::option::Option;

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
use PieceType::{Pawn as P, Knight as N, Bishop as B, Rook as R, Queen as Q, King as K};

//////////////////////////////////////////////////////////////////////////////

enum Piece {
    Black(PieceType),
    White(PieceType)
}
use Piece::{Black as Bl, White as Wh};

//////////////////////////////////////////////////////////////////////////////

type Square = Option<Piece>;

//////////////////////////////////////////////////////////////////////////////

type Board = [Square; 120];

enum SquareIndex {
    NONE,
    A1 = 21, A2, A3, A4, A5, A6, A7, A8,
    B1 = 31, B2, B3, B4, B5, B6, B7, B8,
    C1 = 41, C2, C3, C4, C5, C6, C7, C8,
    D1 = 51, D2, D3, D4, D5, D6, D7, D8,
    E1 = 61, E2, E3, E4, E5, E6, E7, E8,
    F1 = 71, F2, F3, F4, F5, F6, F7, F8,
    G1 = 81, G2, G3, G4, G5, G6, G7, G8,
    H1 = 91, H2, H3, H4, H5, H6, H7, H8,
    I1 = 101, I2, I3, I4, I5, I6, I7, I8
}

const START_BOARD: Board = [
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, Some(Bl(R)), Some(Bl(N)), Some(Bl(B)), Some(Bl(K)), Some(Bl(Q)), Some(Bl(B)), Some(Bl(N)), Some(Bl(R)), None,
    None, Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), None,
    None, Some(Wh(R)), Some(Wh(N)), Some(Wh(B)), Some(Wh(K)), Some(Wh(Q)), Some(Wh(B)), Some(Wh(N)), Some(Wh(R)), None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None
];

fn asAscii(b: Board) -> String {
    return "".to_string()
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn starting_board_as_ascii() {
        use super::START_BOARD;
        use super::asAscii;

        assert!(asAscii(START_BOARD) == "");
    }
}
