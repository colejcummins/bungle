export enum PieceType {
  Pawn = "pawn",
  Rook = "rook",
  Knight = "knight",
  Bishop = "bishop",
  King = "king",
  Queen = "queen"
}

export enum Color {
  White = "white",
  Black = "black"
}

export interface Piece {
  color: Color;
  type: PieceType;
}
