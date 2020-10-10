fen4
====

A Rust library to parse and write fen4 file formats. 

This provides a mapping from a simple representation of a 4 player chess board
and the fen4 file format used by
[Chess.com](https://www.chess.com/4-player-chess). Any non-trivial computation
is likely to have a separate data format, but generates usefulerror messages and
correctly handles almost every variation of 4 player chess (including tons of
non-standard pieces).

### Usage

```
[dependencies]
fen4 = "0.1"
```

### Rust version requirements
Works in stable rust currently, but unclear how far back is supported.

