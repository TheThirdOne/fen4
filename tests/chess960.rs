use fen4::Board;

#[test]
fn default_position() {
    let board = Board::default();
    let board960 = Board::chess960(519);
    println!("{} \n vs \n{} ", board, board960);
    assert_eq!(board, board960, "The default board should be 519")
}

#[test]
fn smallest() {
    let fen = "R-0,0,0,0-1,1,1,1-1,1,1,1-0,0,0,0-0-
3,yR,yK,yR,yN,yN,yQ,yB,yB,3/
3,yP,yP,yP,yP,yP,yP,yP,yP,3/
14/
bR,bP,10,gP,gB/
bK,bP,10,gP,gB/
bR,bP,10,gP,gQ/
bN,bP,10,gP,gN/
bN,bP,10,gP,gN/
bQ,bP,10,gP,gR/
bB,bP,10,gP,gK/
bB,bP,10,gP,gR/
14/
3,rP,rP,rP,rP,rP,rP,rP,rP,3/
3,rB,rB,rQ,rN,rN,rR,rK,rR,3";
    let board: Board = fen.parse().unwrap();
    let board960 = Board::chess960(1);
    println!("{} \n vs \n{} ", board, board960);
    assert_eq!(board, board960);
}
