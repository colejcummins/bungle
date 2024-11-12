use std::fmt;

#[derive(PartialEq, Eq)]
pub enum Color {
  White,
  Black
}

#[derive(PartialEq, Eq)]
pub enum PieceType {
  Pawn,
  Knight,
  Rook,
  Bishop,
  Queen,
  King
}

impl fmt::Display for PieceType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      PieceType::Pawn => write!(f, "P"),
      PieceType::Knight => write!(f, "N"),
      PieceType::Rook => write!(f, "R"),
      PieceType::Bishop => write!(f, "B"),
      PieceType::King => write!(f, "K"),
      PieceType::Queen => write!(f, "Q"),
    }
  }
}

pub struct Piece {
  pub color: Color,
  pub piece_type: PieceType
}

impl fmt::Display for Piece {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", if self.color == Color::White {
      self.piece_type.to_string()
    } else {
      self.piece_type.to_string().to_lowercase()
    })
  }
}

pub type Board = [Option<Piece>; 64];

pub fn display_board(b: Board) -> String {
  b.iter().enumerate().map(|(i, p)| {
    let mut output = "".to_string();
    if i == 0 {
      output.push_str("  a b c d e f g h\n")
    }
    if i % 8 == 0 {
      output.push_str(&((i/8) + 1).to_string());
      output.push_str(" ");
    }
    match p {
      Some(p) => output.push_str(&p.to_string()),
      None => output.push_str(".")
    }
    output.push_str(" ");
    if i % 8 == 7 {
      output.push_str("\n")
    }

    return output
  }).collect::<String>()
}

pub struct CastleState {
  pub white_kingside: bool,
  pub white_queenside: bool,
  pub black_kingside: bool,
  pub black_queenside: bool,
}

pub struct GameState {
  pub board: Board,
  pub player: Color,
  pub castle: CastleState,
  pub en_passant: String,
  pub halfmove: u32,
  pub fullmove: u32,
}
