use std::{collections::HashSet, fmt::Debug};

const ROWS: usize = 8;
const COLS: usize = 8;
const WHITESPACE: char = ' ';
const NOTATION_SEGMENT_COUNT: usize = 6;

#[derive(Default, Debug, PartialEq)]
pub struct CastlingAbility {
  king: bool,
  queen: bool,
}

#[derive(Default, Debug, PartialEq)]
pub struct CastlingStatus {
  white: CastlingAbility,
  black: CastlingAbility,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
  None,
}

impl From<char> for PieceType {
  fn from(chr: char) -> Self {
    match chr {
      'p' | 'P' => Self::Pawn,
      'n' | 'N' => Self::Knight,
      'b' | 'B' => Self::Bishop,
      'r' | 'R' => Self::Rook,
      'q' | 'Q' => Self::Queen,
      'k' | 'K' => Self::King,
      _ => Self::None,
    }
  }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
  Black,
  White,
  None,
}

impl Default for Player {
  fn default() -> Self {
    Self::None
  }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece {
  pub player: Player,
  pub piece: PieceType,
}

impl Piece {
  fn new(player: Player, piece: PieceType) -> Self {
    Self { player, piece }
  }

  fn get_char_repr(&self) -> char {
    let piece = match self.piece {
      PieceType::Pawn => 'p',
      PieceType::Knight => 'n',
      PieceType::Bishop => 'b',
      PieceType::Rook => 'r',
      PieceType::Queen => 'q',
      PieceType::King => 'k',
      PieceType::None => ' ',
    };
    if self.player == Player::White {
      piece.to_ascii_uppercase()
    } else {
      piece
    }
  }
}

impl From<char> for Piece {
  fn from(piece: char) -> Self {
    let player = if piece.is_lowercase() {
      Player::Black
    } else {
      Player::White
    };
    Piece::new(player, PieceType::from(piece))
  }
}

impl CastlingStatus {
  fn set_white_castling_ability(&mut self, piece: &Piece) -> FenResult<()> {
    match piece.piece {
      PieceType::King => self.white.king = true,
      PieceType::Queen => self.white.queen = true,
      _ => return Err(FenParsingError::InvalidCastlingStatus),
    }
    Ok(())
  }

  fn set_black_castling_ability(&mut self, piece: &Piece) -> FenResult<()> {
    match piece.piece {
      PieceType::King => self.black.king = true,
      PieceType::Queen => self.black.queen = true,
      _ => return Err(FenParsingError::InvalidCastlingStatus),
    }
    Ok(())
  }

  fn set_enabled(&mut self, piece: Piece) -> FenResult<()> {
    match piece.player {
      Player::White => self.set_white_castling_ability(&piece)?,
      Player::Black => self.set_black_castling_ability(&piece)?,
      _ => {}
    }
    Ok(())
  }
}

impl Default for Piece {
  fn default() -> Self {
    Piece::new(Player::None, PieceType::None)
  }
}

#[derive(Default, Debug, PartialEq)]
pub struct Fen {
  pub board: Board,
  pub player: Player,
  pub castling_status: CastlingStatus,
  pub enpassent_target: Option<EnpassantTarget>,

  pub halfmove_clock: u8,
  pub fullmove_counter: u8,
}

#[derive(Default, Debug, PartialEq)]
pub struct EnpassantTarget {
  pub file: char,
  pub rank: usize,
}

impl<'a> TryFrom<&'a str> for EnpassantTarget {
  type Error = FenParsingError;

  fn try_from(string: &'a str) -> FenResult<Self> {
    if string.len() != 2 {
      return Err(FenParsingError::InvalidEnpassentTarget);
    }
    let file = string.chars().nth(0).unwrap();
    let rank = string.chars().nth(1).unwrap() as u8 - b'0';

    let is_valid_file = (file as u8) >= (b'a') && (file as u8) <= (b'h');
    if !is_valid_file || !(rank == 3 || rank == 6) {
      return Err(FenParsingError::InvalidEnpassentTarget);
    }
    Ok(Self {
      file,
      rank: rank as usize,
    })
  }
}

type Board = [[Piece; ROWS]; COLS];

#[derive(Debug, PartialEq)]
pub enum FenParsingError {
  RowOverflow,
  ColOverflow,
  InvalidToken,
  InvalidPosition,
  InvalidPlayer,
  InvalidCastlingStatus,
  InvalidEnpassentTarget,
  InvalidNumericValue,
  InsufficentFields,
}

type FenResult<T> = Result<T, FenParsingError>;

impl Fen {
  fn is_valid_piece_pos(pieces: &str) -> FenResult<Board> {
    let (mut curr_row, mut curr_col) = (0, 7);
    let mut board: [[Piece; ROWS]; COLS] = Default::default();
    let mut chars = pieces.chars();

    #[allow(unused_comparisons)]
    loop {
      let current = chars.next().unwrap_or(0 as char);
      if current == 0 as char {
        break;
      }
      match current {
        '/' => {
          if curr_row != ROWS || curr_col == 0 {
            return Err(FenParsingError::InvalidPosition);
          }
          curr_col -= 1;
          curr_row = 0;
        }
        '0'..='9' => {
          let digit = (current as u8 - b'0') as usize;
          let n_row = curr_row + digit;
          if n_row > ROWS {
            return Err(FenParsingError::RowOverflow);
          }
          curr_row = n_row;
        }
        'P' | 'p' | 'K' | 'k' | 'N' | 'n' | 'R' | 'r' | 'Q' | 'q' | 'b' | 'B' => {
          if curr_row >= COLS {
            return Err(FenParsingError::RowOverflow);
          }
          board[curr_col][curr_row] = Piece::from(current);
          curr_row += 1;
        }
        _ => return Err(FenParsingError::InvalidToken),
      }
    }
    if curr_row != 8 || curr_col > 0 {
      return Err(FenParsingError::InvalidPosition);
    }
    Ok(board)
  }

  fn validate_castling_status(string: &str) -> FenResult<CastlingStatus> {
    let mut status = CastlingStatus::default();
    if string == "-" {
      return Ok(status);
    }
    for piece in string
      .chars()
      .collect::<HashSet<char>>()
      .into_iter()
      .map(Piece::from)
    {
      status.set_enabled(piece)?;
    }
    Ok(status)
  }

  fn validate_move_counter(string: &str) -> FenResult<u8> {
    let result = string.parse::<u8>();
    let is_valid_result = result.clone().is_ok_and(|x| (0..=9).contains(&x));
    if !is_valid_result {
      Err(FenParsingError::InvalidNumericValue)
    } else {
      Ok(result.unwrap())
    }
  }

  fn validate((idx, string): (usize, &str), fen: &mut Fen) -> FenResult<()> {
    match idx {
      0 => {
        fen.board = Self::is_valid_piece_pos(string)?;
      }
      1 => {
        if string == "w" {
          fen.player = Player::White;
        } else if string == "b" {
          fen.player = Player::Black;
        } else {
          return Err(FenParsingError::InvalidPlayer);
        }
      }
      2 => fen.castling_status = Self::validate_castling_status(string)?,
      3 => {
        fen.enpassent_target = if string == "-" {
          None
        } else {
          Some(EnpassantTarget::try_from(string)?)
        }
      }
      4 => fen.halfmove_clock = Self::validate_move_counter(string)?,
      5 => fen.fullmove_counter = Self::validate_move_counter(string)?,
      _ => return Ok(()),
    }
    Ok(())
  }

  pub fn parse<T: AsRef<str>>(notation: T) -> FenResult<Fen> {
    let not_ref = notation.as_ref().trim();
    let mut fen = Fen::default();
    if not_ref.is_empty() {
      return Ok(fen);
    }

    let whitespace_count = not_ref
      .chars()
      .map(|x| (x == WHITESPACE) as usize)
      .sum::<usize>();
    if whitespace_count + 1 != NOTATION_SEGMENT_COUNT {
      return Err(FenParsingError::InsufficentFields);
    }
    let split_segments = not_ref.split(WHITESPACE);
    for (i, string) in split_segments.enumerate() {
      Self::validate((i, string), &mut fen)?;
    }
    Ok(fen)
  }

  pub fn board_to_string(&self) -> String {
    let mut notation = String::new();
    let mut space_count = 0;
    let space_count_as_int = |x: u8| (x + b'0') as char;
    for i in (0..self.board.len()).rev() {
      for j in 0..self.board[i].len() {
        let current = &(self.board[i][j]);
        match current.piece {
          PieceType::None => space_count += 1,
          _ => {
            if space_count > 0 {
              notation.push(space_count_as_int(space_count));
              space_count = 0;
            }
            notation.push(current.get_char_repr());
          }
        }
      }
      if space_count > 0 {
        notation.push(space_count_as_int(space_count));
        space_count = 0;
      }
      if i > 0 {
        notation.push('/');
      }
    }
    notation
  }
}

impl TryFrom<String> for Fen {
  type Error = FenParsingError;

  fn try_from(notation: String) -> FenResult<Fen> {
    Self::parse(notation)
  }
}

impl ToString for CastlingStatus {
  fn to_string(&self) -> String {
    let mut string = String::new();
    if self.white.king {
      string.push('K');
    }
    if self.white.queen {
      string.push('Q');
    }
    if self.black.king {
      string.push('k');
    }
    if self.black.queen {
      string.push('q');
    }
    string
  }
}

impl ToString for Fen {
  fn to_string(&self) -> String {
    let mut notation = self.board_to_string();
    notation.push(WHITESPACE);

    notation.push(if self.player == Player::White {
      'w'
    } else {
      'b'
    });
    notation.push(WHITESPACE);
    notation.push_str(self.castling_status.to_string().as_str());
    notation.push(WHITESPACE);
    if let Some(target) = &self.enpassent_target {
      notation.push(target.file);
      notation.push((target.rank as u8 + b'0') as char)
    } else {
      notation.push('-');
    }
    notation.push(WHITESPACE);
    notation.push((self.halfmove_clock + b'0') as char);
    notation.push(WHITESPACE);
    notation.push((self.fullmove_counter + b'0') as char);
    notation
  }
}

#[cfg(test)]
impl From<(bool, bool, bool, bool)> for CastlingStatus {
  fn from(n: (bool, bool, bool, bool)) -> Self {
    Self {
      white: CastlingAbility {
        king: n.0,
        queen: n.1,
      },
      black: CastlingAbility {
        king: n.2,
        queen: n.3,
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{CastlingStatus, Player};

  use super::{EnpassantTarget, Fen};

  #[test]
  fn check_fen() {
    assert_eq!(Fen::parse("    "), Ok(Fen::default()));
    let notation = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Fen::parse(&notation).unwrap();
    assert_eq!(board.player, Player::White);
    assert_eq!(
      board.castling_status,
      CastlingStatus::from((true, true, true, true))
    );
    assert!(board.enpassent_target.is_none());
    assert!(board.halfmove_clock == 0 && board.fullmove_counter == 1);
    assert_eq!(notation, board.to_string());
  }

  #[test]
  fn check_fen_1() {
    let notation = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w Kkq c6 0 2";
    let board = Fen::parse(&notation).unwrap();
    assert_eq!(board.player, Player::White);
    assert_eq!(
      board.castling_status,
      CastlingStatus::from((true, false, true, true))
    );
    assert!(board.enpassent_target == Some(EnpassantTarget { file: 'c', rank: 6 }));
    assert!(board.halfmove_clock == 0 && board.fullmove_counter == 2);
    assert_eq!(
      notation,
      "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w Kkq c6 0 2"
    );
  }

  #[test]
  fn check_fen_2() {
    let notation = "rnbqkbnr/pppppppp/8/8/8/PPPP/QKBNR/PPPPPPPP w KQkq - 0 1";
    let board = Fen::parse(&notation);
    assert_eq!(board, Err(crate::FenParsingError::InvalidPosition));
  }

  #[test]
  fn check_less_fields() {
    let notation = "8/8/8/8 w K - 0 1";
    let board = Fen::parse(&notation);
    assert!(board.is_err());
  }
}
