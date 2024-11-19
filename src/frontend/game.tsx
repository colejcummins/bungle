import React from 'react';

import { Board } from './board';

export const Game = () => {
  return (
    <div className="flex justify-center items-center h-screen w-screen bg-zinc-800">
      <Board />
    </div>
  );
};
