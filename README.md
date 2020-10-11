fen4
====

A Rust library to parse and write fen4 file formats. 

This provides a mapping from a simple representation of a 4 player chess board
and the fen4 file format used by
[Chess.com](https://www.chess.com/4-player-chess). Any non-trivial computation
is likely to have a separate data format, but this generates useful error messages
and correctly handles almost every variation of 4 player chess (including tons of
non-standard pieces).

### Usage

Cargo.toml
```
[dependencies]
fen4 = "0.1"
```

```rust
fn main(){
    let empty_fen = "R-0,0,0,0-0,0,0,0-0,0,0,0-0,0,0,0-0-14/14/14/14/14/14/14/14/14/14/14/14/14/14";
    let board : Result<fen4::Board,fen4::BoardParseError> = empty_fen.parse();
    println!("{}",board.unwrap());
}
```
### Rust version requirements

fen4 requires Rustc version 1.32 or greater. This could certainly be lowered,
but it is unlikely to be needed and would be a non-trivial amount of work.
