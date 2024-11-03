import type { Asset } from './Asset';

// TODO: instead of using an entity, use the corresponding response directly
export interface Tag {
  id: string;
  name: string;
  assets: Asset[];
  createdAt: Date;
  updatedAt: Date;
}
