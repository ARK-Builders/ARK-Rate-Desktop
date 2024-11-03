// TODO: instead of using an entity, use the corresponding response directly
export interface Asset {
  id: string;
  coin: string;
  quantity: number;
  createdAt: Date;
  updatedAt: Date;
}
