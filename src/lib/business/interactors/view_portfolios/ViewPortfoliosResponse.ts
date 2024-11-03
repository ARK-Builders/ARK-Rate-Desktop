interface ResponsePair {
  id: string;
  value: number;
  base: string;
  comparison: string;
  created_at: string;
  updated_at: string;
}

interface ResponseTag {
  id: string;
  name: string;
}

interface ResponseAsset {
  id: string;
  coin: string;
  quantity: number;
}

interface ResponsePortfolio {
  usd_value: number;
  tag: ResponseTag;
  asset: ResponseAsset;
}

export interface ViewPortfoliosResponse {
  usd_pairs: ResponsePair[];
  portfolios: ResponsePortfolio[];
}
