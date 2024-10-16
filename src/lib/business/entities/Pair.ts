import type { Coin } from './Coin';
import type { Entity } from './Entity';

export interface Pair extends Entity {
  base: Coin;
  comparison: Coin;
  value: number; // this represents how much `comparison` values compared to `base`
}
