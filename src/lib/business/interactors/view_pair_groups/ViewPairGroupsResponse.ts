interface ResponsePair {
  id: string;
  value: number;
  base: string;
  comparison: string;
  created_at: string;
  updated_at: string;
}

interface ResponsePairGroup {
  id: string;
  is_pinned: boolean;
  multiplier: number;
  pairs: ResponsePair[];
  created_at: string;
  updated_at: string;
}

export interface ViewPairGroupsResponse {
  usd_pairs: ResponsePair[];
  pair_groups: ResponsePairGroup[];
}
