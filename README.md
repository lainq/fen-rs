# Fenrs

A simple library used to parse Forsyth-Edwards Notation in chess.

```rs
use fenrs::Fen;

fn main() {
  let notation = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  let fen_obj = Fen::parse(&notation);
  let back_to_notation = fen_obj.to_string();
}
```


## Documentation

The library only exposes a handful of structs and functions.
The usage is documented below

```rs
impl Fen {
  // ...
  fn parse<T: AsRef<str>>(notation: T) -> Result<Fen, FenParsingError>;
}

impl ToString for Fen {
  fn to_string(&self) -> String;
}
```

