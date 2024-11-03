import type { Pair } from './Pair';

// TODO: instead of using an entity, use the corresponding response directly
export interface PairGroup {
  id: string;
  pairs: Pair[];
  isPinned: boolean;
  multiplier: number;
  createdAt: Date;
  updatedAt: Date;
}
