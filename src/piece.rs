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
