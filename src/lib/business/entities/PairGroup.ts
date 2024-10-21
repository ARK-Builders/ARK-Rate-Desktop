import type { Pair } from './Pair';

export interface PairGroup {
  id: string;
  pairs: Pair[];
  isPinned: boolean;
  multiplier: number;
  createdAt: Date;
  updatedAt: Date;
}
