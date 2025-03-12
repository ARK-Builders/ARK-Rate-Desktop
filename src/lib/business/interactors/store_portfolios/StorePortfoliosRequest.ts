interface RequestTag {
  id: string;
}

interface RequestAsset {
  coin: string;
  quantity: number;
}

export interface StorePortfoliosRequest {
  tag?: RequestTag;
  assets: RequestAsset[];
}
