interface RequestPair {
  base: string;
  value: number;
  comparison: string;
}

interface RequestPairGroup {
  is_pinned: boolean;
  multiplier: number;
  pairs: RequestPair[];
}

export interface SavePairGroupRequest {
  pair_group: RequestPairGroup;
}
