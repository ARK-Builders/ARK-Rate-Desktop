// TODO: instead of using an entity, use the corresponding response directly
export interface Pair {
  id: string;
  base: string;
  value: number;
  comparison: string;
  createdAt: Date;
  updatedAt: Date;
}
