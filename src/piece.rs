pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub enum Piece {
    Black(Type),
    White(Type)
}

pub fn as_character(p: Piece) -> char {
    return match p {
        Piece::Black(Type::Pawn) => 'P',
        Piece::Black(Type::Knight) => 'N',
        Piece::Black(Type::Bishop) => 'B',
        Piece::Black(Type::Rook) => 'R',
        Piece::Black(Type::Queen) => 'Q',
        Piece::Black(Type::King) => 'K',

        Piece::White(Type::Pawn) => 'p',
        Piece::White(Type::Knight) => 'n',
        Piece::White(Type::Bishop) => 'b',
        Piece::White(Type::Rook) => 'r',
        Piece::White(Type::Queen) => 'q',
        Piece::White(Type::King) => 'k'
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ascii_char_values() {
        use super::{Type, Piece, as_character};

        assert!(as_character(Piece::Black(Type::Pawn)) == 'P');
        assert!(as_character(Piece::Black(Type::Knight)) == 'N');
        assert!(as_character(Piece::Black(Type::Bishop)) == 'B');
        assert!(as_character(Piece::Black(Type::Rook)) == 'R');
        assert!(as_character(Piece::Black(Type::Queen)) == 'Q');
        assert!(as_character(Piece::Black(Type::King)) == 'K');

        assert!(as_character(Piece::White(Type::Pawn)) == 'p');
        assert!(as_character(Piece::White(Type::Knight)) == 'n');
        assert!(as_character(Piece::White(Type::Bishop)) == 'b');
        assert!(as_character(Piece::White(Type::Rook)) == 'r');
        assert!(as_character(Piece::White(Type::Queen)) == 'q');
        assert!(as_character(Piece::White(Type::King)) == 'k');
    }
}
