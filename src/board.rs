use std::ops::Index;

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
use piece;

enum SquareIndex {
    NONE,
    A1 = 21, B1, C1, D1, E1, F1, G1, H1,
    A2 = 31, B2, C2, D2, E2, F2, G2, H2,
    A3 = 41, B3, C3, D3, E3, F3, G3, H3,
    A4 = 51, B4, C4, D4, E4, F4, G4, H4,
    A5 = 61, B5, C5, D5, E5, F5, G5, H5,
    A6 = 71, B6, C6, D6, E6, F6, G6, H6,
    A7 = 81, B7, C7, D7, E7, F7, G7, H7,
    A8 = 91, B8, C8, D8, E8, F8, G8, H8
}

struct Board {
    inner: [square::Square; 120]
}

impl Board {
    fn new() -> Board {
        Board { inner: [
            None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None,
            None, Some(Wh(R)), Some(Wh(N)), Some(Wh(B)), Some(Wh(K)), // 1
                Some(Wh(Q)), Some(Wh(B)), Some(Wh(N)), Some(Wh(R)), None,
            None, Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), // 2
                Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), Some(Wh(P)), None,
            None, None, None, None, None, None, None, None, None, None, // 3
            None, None, None, None, None, None, None, None, None, None, // 4
            None, None, None, None, None, None, None, None, None, None, // 5
            None, None, None, None, None, None, None, None, None, None, // 6
            None, Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), // 7
                Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), Some(Bl(P)), None,
            None, Some(Bl(R)), Some(Bl(N)), Some(Bl(B)), Some(Bl(K)), // 8
                Some(Bl(Q)), Some(Bl(B)), Some(Bl(N)), Some(Bl(R)), None,
            None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None
        ] };
    }

    // Broken
    fn ToString(b: Board) -> String {
        use board::SquareIndex as SI;

        let output_order: [[SquareIndex; 8]; 8] = [
            [SI::A8, SI::B8, SI::C8, SI::D8, SI::E8, SI::F8, SI::G8, SI::H8],
            [SI::A7, SI::B7, SI::C7, SI::D7, SI::E7, SI::F7, SI::G7, SI::H7],
            [SI::A6, SI::B6, SI::C6, SI::D6, SI::E6, SI::F6, SI::G6, SI::H6],
            [SI::A5, SI::B5, SI::C5, SI::D5, SI::E5, SI::F5, SI::G5, SI::H5],
            [SI::A4, SI::B4, SI::C4, SI::D4, SI::E4, SI::F4, SI::G4, SI::H4],
            [SI::A3, SI::B3, SI::C3, SI::D3, SI::E3, SI::F3, SI::G3, SI::H3],
            [SI::A2, SI::B2, SI::C2, SI::D2, SI::E2, SI::F2, SI::G2, SI::H2],
            [SI::A1, SI::B1, SI::C1, SI::D1, SI::E1, SI::F1, SI::G1, SI::H1]
        ];

        // 64 squares, 8 newlines
        let mut ret = String::with_capacity(72);
        for rank in output_order.iter() {
            for si in rank.iter() {
                ret.push(square::as_character(b[si]));
            }
        }
        
        return ret;
    }
}

impl Index<SquareIndex> for Board {
    type Output = square::Square;

    // Broken
    fn index<'a>(&'a self, _index: SquareIndex) -> &'a square::Square {
        return None;
    }
}

// Todo: FEN output
// Todo: FEN input

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn starting_board_as_ascii() {
        // Todo: Tidy this string representation up
        assert!(Board::new().to_string() == "RNBQKBNR
PPPPPPPP
._._._._
_._._._.
._._._._
_._._._.
pppppppp
rnbqkbnr
");
    }
}
