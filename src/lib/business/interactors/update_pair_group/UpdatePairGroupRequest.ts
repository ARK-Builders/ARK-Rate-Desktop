interface RequestPair {
  id: string;
  base: string;
  value: number;
  comparison: string;
}

interface RequestPairGroup {
  id: string;
  is_pinned: boolean;
  multiplier: number;
  pairs: RequestPair[];
}

export interface UpdatePairGroupRequest {
  pair_group: RequestPairGroup;
}
