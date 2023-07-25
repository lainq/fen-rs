# Fenc

A simple library used to parse Forsyth-Edwards Notation in chess.

```c
#include "fenc.h"

int main() {  
  const char* notation =  '...';
  struct fen fen_obj = fen_parse_notation(notation);
  printf("Can enpassent?: %s", fen_obj.is_enpassent_target_none ? "no": "yes"); 
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