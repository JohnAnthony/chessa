use square;

use piece::Type::{
    Pawn as P,
    Knight as N,
    Bishop as B,
    Rook as R,
    Queen as Q,
    King as K
};
use piece::Piece::{Black as Bl, White as Wh};

type Board = [square::Square; 120];

enum SquareIndex {
    NONE,
    A1 = 21, B1, C1, D1, E1, F1, H1,
    A2 = 31, B2, C2, D2, E2, F2, H2,
    A3 = 41, B3, C3, D3, E3, F3, H3,
    A4 = 51, B4, C4, D4, E4, F4, H4,
    A5 = 61, B5, C5, D5, E5, F5, H5,
    A6 = 71, B6, C6, D6, E6, F6, H6,
    A7 = 81, B7, C7, D7, E7, F7, H7,
    A8 = 91, B8, C8, D8, E8, F8, H8
}

const START_BOARD: Board = [
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, Some(Wh(R)), Some(Wh(N)), Some(Wh(B)), Some(Wh(K)), Some(Wh(Q)), Some(Wh(B)), Some(Wh(N)), Some(Wh(R)), None,
    None, Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None,
    None, Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), None,
    None, Some(Bl(R)), Some(Bl(N)), Some(Bl(B)), Some(Bl(K)), Some(Bl(Q)), Some(Bl(B)), Some(Bl(N)), Some(Bl(R)), None,
    None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None
];

fn asAscii(b: Board) -> String {
    return "".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn starting_board_as_ascii() {
        use super::START_BOARD;
        use super::asAscii;

        let chk = "RNBQKBNR\
PPPPPPPP
._._._._
_._._._.
._._._._
_._._._.
pppppppp
rnbqkbnr";

        assert!(asAscii(START_BOARD) == chk);
    }
}
