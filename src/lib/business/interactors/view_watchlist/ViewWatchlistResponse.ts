interface ResponsePair {
  id: string;
  base: string;
  value: number;
  comparison: string;
  created_at: string;
  updated_at: string;
}

interface ResponsePairCombination {
  value: number;
  comparison: string;
  fluctuation: number;
}

interface ResponseWatchlistPair {
  base: ResponsePair;
  combinations: ResponsePairCombination[];
}

export interface ViewWatchlistResponse {
  coins: string[];
  pairs: ResponseWatchlistPair[];
}
