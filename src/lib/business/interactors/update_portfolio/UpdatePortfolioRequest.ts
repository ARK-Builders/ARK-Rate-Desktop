interface RequestAsset {
  id: string;
  coin: string;
  quantity: number;
}

export interface UpdatePortfolioRequest {
  tag_ids: string[];
  asset: RequestAsset;
}
