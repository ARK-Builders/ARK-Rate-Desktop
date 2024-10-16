import type { Entity } from './Entity';
import type { Pair } from './Pair';

export interface PairGroup extends Entity {
  pairs: Pair[];
}
