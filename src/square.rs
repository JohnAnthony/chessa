use std::option::Option;
use piece;

pub type Square = Option<piece::Piece>;

pub fn as_character(sq: Square) -> char {
    return match sq {
        None => '.',
        Some(p) => piece::as_character(p)
    }
}
