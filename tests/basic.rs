use fen4::Board;
#[test]
fn default() {
    let default_fen = "R-0,0,0,0-1,1,1,1-1,1,1,1-0,0,0,0-0-
3,yR,yN,yB,yK,yQ,yB,yN,yR,3/
3,yP,yP,yP,yP,yP,yP,yP,yP,3/
14/
bR,bP,10,gP,gR/
bN,bP,10,gP,gN/
bB,bP,10,gP,gB/
bK,bP,10,gP,gQ/
bQ,bP,10,gP,gK/
bB,bP,10,gP,gB/
bN,bP,10,gP,gN/
bR,bP,10,gP,gR/
14/
3,rP,rP,rP,rP,rP,rP,rP,rP,3/
3,rR,rN,rB,rQ,rK,rB,rN,rR,3";
    let board: Board = default_fen.parse().unwrap();
    let string = board.to_string();
    assert_eq!(default_fen, string, "FromStr and Display are not inverses");
    let board2: Board = string.parse().unwrap();
    assert_eq!(board, board2, "FromStr and Display are not inverses2");
}

#[test]
fn complicated() {
    let complicated_fen = "R-0,0,0,0-0,0,0,0-0,0,0,0-0,0,0,0-0-{'lives':(50,50,50,50),'enPassant':('i3:i4','c6:d6','f12:f11','l9:k9')}-
3,yA,yP,yN,yB,yR,yD,yQ,yK,3/
3,yE,yH,1,yC,yV,yG,yF,yW,3/
3,yJ,yL,1,yβ,yα,yY,yS,yI,3/
bK,bW,bI,2,yP,yT,yZ,yO,2,gJ,gE,gA/
bQ,bF,bS,3,yδ,yγ,yM,2,gL,gH,gP/
bD,bG,bY,bO,bM,dK,dQ,dD,dR,1,gP,2,gN/
bR,bV,bα,bZ,bγ,dB,dN,dP,X,gδ,gT,gβ,gC,gB/
bB,bC,bβ,bT,bδ,dF,dL,dJ,dT,gγ,gZ,gα,gV,gR/
bN,2,bP,1,dδ,dγ,dα,dZ,gM,gO,gY,gG,gD/
bP,bH,bL,2,rM,rγ,rδ,3,gS,gF,gQ/
bA,bE,bJ,2,rO,rZ,rT,rP,2,gI,gW,gK/
3,rI,rS,rY,rα,rβ,1,rL,rJ,3/
3,rW,rF,rG,rV,rC,1,rH,rE,3/
3,rK,rQ,rD,rR,rB,rN,rP,rA,3";
    let board: Board = complicated_fen.parse().unwrap();
    println!("{}", complicated_fen.len());
    let string = board.to_string();
    assert_eq!(
        complicated_fen, string,
        "FromStr and Display are not inverses"
    );
    let board2: Board = string.parse().unwrap();
    assert_eq!(board, board2, "FromStr and Display are not inverses2");
}
