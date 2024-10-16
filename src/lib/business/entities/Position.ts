import type { Coin } from './Coin';
import type { Entity } from './Entity';

export interface Position extends Entity {
  coin: Coin;
  quantity: number;
}
