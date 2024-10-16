import type { Entity } from './Entity';
import type { Position } from './Position';

export interface Portfolio extends Entity {
  positions: Position[];
}
