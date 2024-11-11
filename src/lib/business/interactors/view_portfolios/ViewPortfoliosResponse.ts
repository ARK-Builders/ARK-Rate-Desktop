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
  created_at: string;
  updated_at: string;
}

interface ResponseAsset {
  id: string;
  coin: string;
  quantity: number;
  usd_value: number;
  created_at: string;
  updated_at: string;
}

interface ResponsePortfolio {
  fluctuation: number;
  tags: ResponseTag[];
  asset: ResponseAsset;
}

export interface ViewPortfoliosResponse {
  tags: ResponseTag[];
  usd_pairs: ResponsePair[];
  portfolios: ResponsePortfolio[];
}
