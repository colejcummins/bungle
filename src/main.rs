mod structs;

const DEFAULT_FEN_STRING: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn decode_castle_state(castle: String) -> structs::CastleState {
  return structs::CastleState {
    white_kingside: castle.contains('K'),
    white_queenside: castle.contains('Q'),
    black_kingside: castle.contains('k'),
    black_queenside: castle.contains('q'),
  };
}

fn decode_board_string(board_string: String) -> structs::Board {
  let mut board_ind: usize = 0;

  const DEFAULT: Option<structs::Piece> = None;
  let mut board: structs::Board = [DEFAULT; 64];

  for c in board_string.chars() {
    let color = if c.is_uppercase() { structs::Color::White } else { structs::Color::Black };
    match c {
      _ if c.is_digit(10) => {
        board_ind += (c.to_digit(10).unwrap_or(0) as usize) - 1;
      }
      'P' | 'p' => board[board_ind] = Some(structs::Piece {
        color: color,
        piece_type: structs::PieceType::Pawn,
      }),
      'K' | 'k' => board[board_ind] = Some(structs::Piece {
        color: color,
        piece_type: structs::PieceType::King,
      }),
      'Q' | 'q' => board[board_ind] = Some(structs::Piece {
        color: color,
        piece_type: structs::PieceType::Queen,
      }),
      'B' | 'b' => board[board_ind] = Some(structs::Piece {
        color: color,
        piece_type: structs::PieceType::Bishop,
      }),
      'N' | 'n' => board[board_ind] = Some(structs::Piece {
        color: color,
        piece_type: structs::PieceType::Knight,
      }),
      'R' | 'r' => board[board_ind] = Some(structs::Piece {
        color: color,
        piece_type: structs::PieceType::Rook,
      }),
      '/' => continue,
      _ => ()
    }

    board_ind += 1;
  }

  return board;
}

// Accepts a valid fen string, returns a game state
fn decode_fen_string(fen: String) -> Result<structs::GameState, &'static str> {
  let fen_vec = fen.split_whitespace().collect::<Vec<&str>>();

  if let [board, player, castle, en_passant, halfmove, fullmove] = &fen_vec[..] {
    return Ok(structs::GameState {
      board: decode_board_string(board.to_string()),
      player: if player.chars().next().unwrap_or('w') == 'w' { structs::Color::White } else { structs::Color::Black },
      castle: decode_castle_state(castle.to_string()),
      en_passant: en_passant.to_string(),
      halfmove: halfmove.parse::<u32>().unwrap(),
      fullmove: fullmove.parse::<u32>().unwrap(),
    });
  } else {
    Err("Malformed fen string")
  }
}

fn valid_knight_moves(ind: usize, board: &structs::Board) -> Vec<usize> {
  let cur_color = board[ind].as_ref().unwrap().color;
  let mut moves = Vec::<usize>::new();

  let diff_color = |new_ind: usize| -> bool {
    match &board[new_ind] {
      Some(p) => p.color != cur_color,
      None => true
    }
  };

  let mod_ind = ind % 8;

  // top L moves
  if ind + 17 < 64 && mod_ind < 7 && diff_color(ind + 17) {
    moves.push(ind + 17)
  }
  if ind + 15 < 64 && mod_ind > 0 && diff_color(ind + 15) {
    moves.push(ind + 15)
  }

  // bottom L moves
  if ind - 15 >= 0 && mod_ind < 7 && diff_color(ind - 15) {
    moves.push(ind - 15)
  }
  if ind - 17 >= 0 && mod_ind > 0 && diff_color(ind - 17) {
    moves.push(ind - 17)
  }

  // top row moves
  return moves;
}

fn main() {

  if let Ok(state) = decode_fen_string(DEFAULT_FEN_STRING.to_string()) {
    println!("{}", structs::display_board(state.board))
  }
}
