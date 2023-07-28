#include "../include/fenc.h"
#include <assert.h>
#include <stdio.h>
#include <string.h>

struct test_case {
  const char* notation;
  struct fen data;
};

struct fen fen_for_test(enum fen_player player, char* ep, int wk, int wq, int bk, int bq) {
  struct fen fobj = fen_new();
  fobj.castling.black = (struct fen_castling_ability) { .king = bk, .queen = bq };
  fobj.castling.white = (struct fen_castling_ability) { .king = wk, .queen = wq };
  fobj.player = player;
  if(ep == NULL) {
    fobj.is_enpassent_target_none = 1;
  } else {
    fobj.is_enpassent_target_none = 0;
    fobj.enpassent = (struct fen_enpassent_target) {
      .rank = ep[1] - '0',
      .file = ep[0]
    };  }

  return fobj;
}

#define COMPARE_CASTLING_ABILITY(curr, cmp, player)\
(curr->castling.player.king == cmp->castling.player.king) &&\
(curr->castling.player.queen == cmp->castling.player.queen)

#define COMPARE_ENPASSENT(curr, cmp)\
(curr->enpassent.file == cmp->enpassent.file) &&\
(curr->enpassent.rank == cmp->enpassent.rank)

int compare_fen_properties(const struct fen* curr, const struct fen* cmp) {
  int result = COMPARE_CASTLING_ABILITY(curr, cmp, black) && 
    COMPARE_CASTLING_ABILITY(curr, cmp, white) &&
    (curr->player == cmp->player) &&
    (curr->is_enpassent_target_none == cmp->is_enpassent_target_none);
  if(!curr->is_enpassent_target_none) {
    return result && COMPARE_ENPASSENT(curr, cmp);
  }
  return result;
}

#define GENERATE_TEST_CASE(ntn, player, enp, c1, c2, c3, c4)\
(struct test_case) {\
.notation = ntn,\
.data = fen_for_test(player, enp, c1, c2, c3, c4)}

#define PLAYER_STRING(player)\
player == FEN_PLAYER_WHITE ? "white" : "black"
#define PRINT_ENPASSENT(fobj)\
if(fobj->is_enpassent_target_none){\
printf("None ");} else{\
printf("%c%u ", fobj->enpassent.file, \
fobj->enpassent.rank);}

#define CASTLING_DATA(fobj)\
fobj->castling.white.king, fobj->castling.white.queen,\
fobj->castling.black.king, fobj->castling.black.queen

void fen_debug_print(const struct fen* a, const struct fen* b) {
  printf("Player:%s | %s\n", PLAYER_STRING(a->player), PLAYER_STRING(b->player));
  printf("Enpassent: ");
    PRINT_ENPASSENT(a);
    PRINT_ENPASSENT(b);
  printf("\n");
  printf("castling %d %d %d %d | %d %d %d %d\n",
    CASTLING_DATA(a), CASTLING_DATA(b));
}

#define test_cases_c (size_t) 4
int main() {
  const struct test_case test_cases[test_cases_c] = {
    GENERATE_TEST_CASE("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      FEN_PLAYER_WHITE, NULL, 1, 1, 1, 1),
    GENERATE_TEST_CASE("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 1",
      FEN_PLAYER_WHITE, "c6", 1, 1, 1, 1),
    GENERATE_TEST_CASE("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
      FEN_PLAYER_BLACK, "e3", 1, 1, 1, 1),
    GENERATE_TEST_CASE("8/8/8/8/8/8/8/8 w Qk - 0 1",
  FEN_PLAYER_WHITE, NULL, 0, 1, 1, 0)
  };
  struct fen fobj;
  size_t success = 0;
  for(size_t i=0; i<test_cases_c; ++i) {
    struct test_case curr = test_cases[i];
    fobj = fen_parse_notation(curr.notation);
    printf("[%zu:%zu] ", i + 1, test_cases_c);

    char* cback = fen_notation_from_fen(&fobj);

    int cmp_properties = compare_fen_properties(&fobj, &(curr.data));
    if(cmp_properties && 
      (strcmp(cback, curr.notation) == 0)) {
      printf("Success\n");
      ++success;
    } else {
printf("failed\n");
      if(!cmp_properties) {
        printf("Error while comparing properties\n");
        fen_debug_print(&fobj, &(curr.data));
      } else {
        printf("Expected %s, got %s\n",
          curr.notation, cback);
      }
    }
  }
  printf("Passed %zu out of %zu. Failed %zu\n", success, test_cases_c, 
    test_cases_c - success);
}
