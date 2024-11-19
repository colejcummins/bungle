import React from 'react';
import cx from 'classnames';

import { Piece, Color, PieceType } from './types';
import { PieceIcon } from './piece-icon';

const cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

const defaultBoard: Array<Piece | null> = [
  { color: Color.Black, type: PieceType.Rook },
  { color: Color.Black, type: PieceType.Knight },
  { color: Color.Black, type: PieceType.Bishop },
  { color: Color.Black, type: PieceType.Queen },
  { color: Color.Black, type: PieceType.King },
  { color: Color.Black, type: PieceType.Bishop },
  { color: Color.Black, type: PieceType.Knight },
  { color: Color.Black, type: PieceType.Rook },
  { color: Color.Black, type: PieceType.Pawn },
  { color: Color.Black, type: PieceType.Pawn },
  { color: Color.Black, type: PieceType.Pawn },
  { color: Color.Black, type: PieceType.Pawn },
  { color: Color.Black, type: PieceType.Pawn },
  { color: Color.Black, type: PieceType.Pawn },
  { color: Color.Black, type: PieceType.Pawn },
  { color: Color.Black, type: PieceType.Pawn },
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Pawn },
  { color: Color.White, type: PieceType.Rook },
  { color: Color.White, type: PieceType.Knight },
  { color: Color.White, type: PieceType.Bishop },
  { color: Color.White, type: PieceType.Queen },
  { color: Color.White, type: PieceType.King },
  { color: Color.White, type: PieceType.Bishop },
  { color: Color.White, type: PieceType.Knight },
  { color: Color.White, type: PieceType.Rook }
];

const Cell = ({piece, rowNum, cellNum}: {piece: Piece | null, rowNum: number, cellNum: number}) => {
  return (
    <div
    className={cx(`flex justify-center align-center relative w-[96px] h-[96px]`, ((rowNum % 8) + cellNum) % 2 === 1 ? 'bg-zinc-400' : 'bg-zinc-100')}
  >
    {cellNum % 8 === 0 && (
      <div
        className={cx(
          'absolute font-mono text-lg top-1 left-1 select-none',
          rowNum % 2 === 0 ? 'text-zinc-400' : 'text-zinc-100'
        )}
      >
        {Math.floor(cellNum/8)}
      </div>
    )}
    {rowNum === 7 && (
      <div
        className={cx(
          'absolute font-mono text-lg right-2 bottom-0 select-none',
          cellNum % 2 === 1 ? 'text-zinc-400' : 'text-zinc-100'
        )}
      >
        {cols[cellNum % 8]}
      </div>
    )}
      {piece && <PieceIcon {...piece} width={72} />}
  </div>
  )
}

export const Board = () => {
  const renderRow = (rowNum: number) => {
    const outputCells = [];

    for (let cell = rowNum * 8; cell < (rowNum + 1) * 8; cell += 1) {
      outputCells.push(
        <Cell piece={defaultBoard[cell]} cellNum={cell} rowNum={rowNum} />
      );
    }
    return <div className="flex">{outputCells}</div>;
  };

  return <div className="flex flex-col">{Array(8).fill(0).map((_, i) => renderRow(i))}</div>;
};
