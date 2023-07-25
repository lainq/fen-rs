#ifndef FEN_H_INCLUDED
#define FEN_H_INCLUDED

#include <stdlib.h>
#include <string.h>

#define ROWS 8
#define COLS 8

struct fen_cstrok_result {
  unsigned int offset;
  unsigned int size;
  int is_last;
};

struct fen_castling_ability {
  int king;
  int queen;
};

struct fen_castling_status {
  struct fen_castling_ability white;
  struct fen_castling_ability black;
};

static struct fen_cstrok_result cstrtok(const char* str, char delim) {
  static unsigned int last_occ = 0;

  unsigned int offset = last_occ > 0 ? last_occ + 1 : 0;
  int is_last = 0;
  for (unsigned int i = offset;; ++i) {
    if (str[i] == delim || str[i] == '\0') {
      last_occ = i;
      is_last = str[i] == '\0';
      break;
    }
  }
  struct fen_cstrok_result result = {
      .offset = offset, .size = last_occ - offset, .is_last = is_last};
  return result;
}

static char fen_cmp_pieces[6] = {'R', 'B', 'K', 'N', 'P', 'Q'};
static unsigned pieces_size = 6;

static int fen_is_valid_piece(const char piece) {
  char cmp_piece = piece >= 97 ? piece - 32 : piece;
  for (unsigned i = 0; i < pieces_size; ++i) {
    if (cmp_piece == fen_cmp_pieces[i]) return 1;
  }
  return 0;
}

enum fen_player { FEN_PLAYER_BLACK, FEN_PLAYER_WHITE };

struct fen_enpassent_target {
  char file;
  unsigned int rank;
};

struct fen {
  char board[COLS][ROWS];
  enum fen_player player;
  struct fen_castling_status castling;
  struct fen_enpassent_target enpassent;
  int is_enpassent_target_none;
  int halfmove_clock;
  int fullmove_counter;
};

#include <assert.h>

static struct fen fen_validate_piece_position(const char* token,
                                          unsigned int size) {
  struct fen fen_pieces;
  unsigned col_idx = 7, row_idx = 0;
  for (unsigned i = 0; i < size; ++i) {
    char curr = token[i];
    switch (curr) {
      case '/':
        --col_idx;
        row_idx = 0;
        break;
      case '0' ... '9': {
        unsigned row_idx_e = row_idx + (curr - '0') - 1;
        assert(row_idx_e <= ROWS);
        for (; row_idx <= row_idx_e; ++row_idx) {
          fen_pieces.board[col_idx][row_idx] = ' ';
        }
      } break;
      default:
        if (fen_is_valid_piece(curr)) {
          assert(ROWS >= row_idx);
          fen_pieces.board[col_idx][row_idx++] = curr;
        } else {
          assert(0 && "Invalid character");
        }
        break;
    }
  }
  // The last column isnt added because we do not
  // increment col_idx as there is no '/' at the end
  assert(col_idx == 0 && row_idx == ROWS);
  return fen_pieces;
}

// Assumtions made: Length is already verified to be 2
static int fen_validate_enpassent_target(const char* value) {
  return (value[0] >= 'a' && value[0] <= 'h') &&
         (value[1] == '3' || value[1] == '6');
}

static int fen_is_valid_player(char* player) {
  return (strcmp(player, "w") == 0) || strcmp(player, "b") == 0;
}

#define FEN_DEFAULT_CASTLING_ABILITY() \
  (struct fen_castling_ability) { .king = 0, .queen = 0 }

static struct fen_castling_status get_fen_castling_status(char* value) {
  struct fen_castling_status status = {.white = FEN_DEFAULT_CASTLING_ABILITY(),
                                       .black = FEN_DEFAULT_CASTLING_ABILITY()};
  if (value[0] == '-' && value[1] == '\0') {
    goto ret;
  }
  for (char* vcopy = value; *vcopy != '\0'; ++vcopy) {
    switch (*vcopy) {
      case 'K':
        status.white.king = 1;
        break;
      case 'k':
        status.black.king = 1;
        break;
      case 'q':
        status.black.queen = 1;
        break;
      case 'Q':
        status.white.queen = 1;
        break;
      default:
        assert(0 && "Invalid character in castling data");
    }
  }
ret:
  return status;
}

#undef FEN_DEFAULT_CASTLING_ABILITY

static int fen_parse_number(const char* str, size_t size) {
  int power = (size - 1) * 10, n = 0;
  for (size_t i = 0; i < size; ++i) {
    assert(str[i] >= '0' && str[i] <= '9');
    n += str[i] - '0';
    if (power > 0) n *= power;
    power /= 10;
  }
  return n;
}

#define WHITESPACE ' '

struct fen fen_parse_notation(const char* notation) {
  struct fen_cstrok_result tok_res = cstrtok(notation, WHITESPACE);
  unsigned int curr_wi = 0;
  struct fen fen_obj;

  while (++curr_wi > 0) {
    if (tok_res.size > 0) {
      char token[tok_res.size + 1];
      token[tok_res.size] = '\0';
      memcpy(token, notation + tok_res.offset, tok_res.size);
      switch (curr_wi) {
        case 1:
          fen_obj = fen_validate_piece_position(token, tok_res.size);
          break;
        case 2:
          assert(fen_is_valid_player(token));
          fen_obj.player =
              token[0] == 'w' ? FEN_PLAYER_WHITE : FEN_PLAYER_BLACK;
          break;
        case 3:
          fen_obj.castling = get_fen_castling_status(token);
          break;
        case 4:
          if (tok_res.size == 1 && token[0] == '-') {
            fen_obj.is_enpassent_target_none = 1;
          } else {
            assert(tok_res.size == 2 && fen_validate_enpassent_target(token));
            fen_obj.enpassent.file = token[0];
            fen_obj.enpassent.rank = token[1] - '0';
          }
          break;
        case 5:
          fen_obj.halfmove_clock = fen_parse_number(token, tok_res.size);
          break;
        case 6:
          fen_obj.fullmove_counter = fen_parse_number(token, tok_res.size);
          break;
        default:
          break;
      }

    } else {
      --curr_wi;
    }  
    if (tok_res.is_last) break;
    tok_res = cstrtok(notation, WHITESPACE);
  }
  assert(curr_wi == 6);
  return fen_obj;
}

#undef WHITESPACE
#undef ROWS
#undef COLS

#endif


