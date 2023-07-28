# Fenc

A simple library used to parse Forsyth-Edwards Notation in chess.

```c
#include <string.h>
#include <assert.h>
#include "fenc.h"

#define NOTATION "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"

int main() {  
  struct fen fen_obj = fen_parse_notation(NOTATION);
  printf("Can enpassent?: %s", fen_obj.is_enpassent_target_none ? "no": "yes"); 
  // yes
  
  char* new_notation = fen_notation_from_fen(&fen_obj); // returns a heap allocated string
  // rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1
  assert(strcmp(new_notation, NOTATION) == 0);
  return 0;
}
```


## Documentation

The library only exposes a handful of structs and functions.
The usage is documented below

```
struct fen fen_parse_notation(const char* notation);
```


```c
struct fen {
  char board[COLS][ROWS];
  // FEN_PLAYER_BLACK or FEN_PLAYER_WHITE
  enum fen_player player;
  struct fen_castling_status castling;
  struct fen_enpassent_target enpassent;
  // Whether an enpassent target exists or not
  int is_enpassent_target_none;
  int halfmove_clock;
  int fullmove_counter;
};
```

#### `struct fen_enpassent_target`

```c

struct fen_enpassent_target {
  char file;
  unsigned int rank;
};
```

#### `struct fen_castling_status`
```c
struct fen_castling_ability {
  int king;
  int queen;
};

struct fen_castling_status {
  struct fen_castling_ability white;
  struct fen_castling_ability black;
};
```