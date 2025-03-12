interface ResponsePair {
  id: string;
  fluctuation: number;
  base: string;
  value: number;
  comparison: string;
  created_at: string;
  updated_at: string;
}

export interface ViewWatchlistResponse {
  coins: string[];
  pairs: ResponsePair[];
}
