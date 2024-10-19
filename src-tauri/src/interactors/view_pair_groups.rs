use chrono::Utc;
use serde::Serialize;

use crate::{
    entities::{pair::Pair, pair_group::PairGroup},
    utilities::coin_market::CoinMarket,
    Error,
};

use super::interactor::Interactor;

pub trait ViewPairGroupsDataAccess {
    async fn fetch_pair_groups(&mut self) -> Result<Vec<PairGroup>, Error>;
    async fn update_pair_group(&mut self, pair_group: &PairGroup) -> Result<(), Error>;
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponsePair {
    pub id: String,
    pub value: f64,
    pub base: String,
    pub comparison: String,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for ResponsePair {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.base == other.base
            && self.value == other.value
            && self.comparison == other.comparison
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponsePairGroup {
    pub id: String,
    pub is_pinned: bool,
    pub pairs: Vec<ResponsePair>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for ResponsePairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.is_pinned == other.is_pinned
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ViewPairGroupsResponse {
    pub pair_groups: Vec<ResponsePairGroup>,
}

pub struct ViewPairGroups<DA, CM> {
    pub data_access: DA,
    pub coin_market: CM,
}

impl<DA, CM> Interactor<(), ViewPairGroupsResponse> for ViewPairGroups<DA, CM>
where
    DA: ViewPairGroupsDataAccess,
    CM: CoinMarket,
{
    async fn perform(&mut self, _request: ()) -> Result<ViewPairGroupsResponse, Error> {
        let mut pair_groups: Vec<PairGroup> = vec![];
        let fresh_pairs: Vec<Pair> = self.coin_market.retrieve_pairs_by_base("USD").await?;
        let stored_pair_groups: Vec<PairGroup> = self.data_access.fetch_pair_groups().await?;
        for stored_pair_group in &stored_pair_groups {
            if stored_pair_group.is_pinned {
                let fresh_pair_group = refresh_pair_group(&fresh_pairs, stored_pair_group);
                self.data_access
                    .update_pair_group(&fresh_pair_group)
                    .await?;
                pair_groups.push(fresh_pair_group);
            } else {
                pair_groups.push(stored_pair_group.clone())
            }
        }
        return Ok(ViewPairGroupsResponse {
            pair_groups: pair_groups
                .iter()
                .map(|pg| ResponsePairGroup {
                    id: pg.id.clone(),
                    is_pinned: pg.is_pinned,
                    pairs: pg
                        .pairs
                        .iter()
                        .map(|p| ResponsePair {
                            id: p.id.clone(),
                            base: p.base.clone(),
                            value: p.value.clone(),
                            comparison: p.comparison.clone(),
                            created_at: p.created_at.clone(),
                            updated_at: p.updated_at.clone(),
                        })
                        .collect(),
                    created_at: pg.created_at.clone(),
                    updated_at: pg.updated_at.clone(),
                })
                .collect(),
        });
    }
}

fn refresh_pair_group(fresh_pairs: &Vec<Pair>, pair_group: &PairGroup) -> PairGroup {
    let mut fresh_pair_group = PairGroup {
        id: pair_group.id.clone(),
        pairs: vec![],
        is_pinned: pair_group.is_pinned,
        created_at: pair_group.created_at.clone(),
        updated_at: Utc::now().to_rfc3339(),
    };
    for pair in &pair_group.pairs {
        for fresh_pair in fresh_pairs {
            let has_same_base: bool = pair.base == fresh_pair.base;
            let has_same_comparison: bool = pair.comparison == fresh_pair.comparison;
            if has_same_base && has_same_comparison {
                fresh_pair_group.pairs.push(Pair {
                    id: pair.id.clone(),
                    base: pair.base.clone(),
                    value: fresh_pair.value.clone(),
                    comparison: pair.comparison.clone(),
                    created_at: pair.created_at.clone(),
                    updated_at: fresh_pair.updated_at.clone(),
                });
                break;
            }
        }
    }
    return fresh_pair_group;
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Utc;

    pub struct ViewPairGroupsDataAccessMock {
        fetch_pair_groups_result: Result<Vec<PairGroup>, Error>,
        update_pair_group_result: Result<(), Error>,
    }

    impl ViewPairGroupsDataAccess for ViewPairGroupsDataAccessMock {
        async fn fetch_pair_groups(&mut self) -> Result<Vec<PairGroup>, Error> {
            return self.fetch_pair_groups_result.clone();
        }

        async fn update_pair_group(&mut self, _pair_group: &PairGroup) -> Result<(), Error> {
            return self.update_pair_group_result.clone();
        }
    }

    pub struct CoinMarketMock {
        retrieve_pairs_by_base_result: Result<Vec<Pair>, Error>,
    }

    impl CoinMarket for CoinMarketMock {
        async fn retrieve_pairs_by_base(&mut self, _base: &str) -> Result<Vec<Pair>, Error> {
            return self.retrieve_pairs_by_base_result.clone();
        }
    }

    #[tokio::test]
    async fn test_perform_success() {
        /*
            Expectations for each example pair group (contained in 'example_pair_groups'):

            [0]: This is a pinned pair scenario.
                - It should receive new pairs, based on 'fresh_example_pairs'.
                - The new pairs should have the same ID's as their matches from the old pair that has the same base and comparison coins.
                - 'created_at' for the pair group and contained pairs should remain the same.
                - 'updated_at' for the pair group should be updated.
                - 'updated_at' for the contained pairs should be the same as their matching fresh pairs.

            [1]: This is a non-pinned pair scenario.
                - It should just receive the old pairs.
                - Both 'created_at' and 'updated_at' for the pair group and contained pairs should remain the same.
        */

        let example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 1.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p2".to_string(),
                value: 2.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p3".to_string(),
                value: 3.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        ];

        let fresh_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p4".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p5".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            Pair {
                id: "p6".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        ];

        let example_pair_groups: Vec<PairGroup> = vec![
            PairGroup {
                id: "pg1".to_string(),
                is_pinned: true,
                pairs: vec![example_pairs[0].clone(), example_pairs[1].clone()],
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
            PairGroup {
                id: "pg2".to_string(),
                is_pinned: false,
                pairs: vec![example_pairs[2].clone()],
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            },
        ];

        let data_access = ViewPairGroupsDataAccessMock {
            fetch_pair_groups_result: Ok(vec![
                example_pair_groups[0].clone(),
                example_pair_groups[1].clone(),
            ]),
            update_pair_group_result: Ok(()),
        };

        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_example_pairs.clone()),
        };

        let mut interactor = ViewPairGroups {
            data_access,
            coin_market,
        };
        let response = interactor.perform(()).await.unwrap();

        assert_eq!(response.pair_groups.len(), 2);

        // pair_group1
        assert_eq!(
            response.pair_groups[0],
            ResponsePairGroup {
                id: example_pair_groups[0].id.clone(),
                is_pinned: true,
                pairs: vec![
                    ResponsePair {
                        id: example_pairs[0].id.clone(),
                        base: example_pairs[0].base.clone(),
                        value: fresh_example_pairs[0].value.clone(),
                        comparison: example_pairs[0].comparison.clone(),
                        created_at: example_pairs[0].created_at.clone(),
                        updated_at: fresh_example_pairs[0].updated_at.clone(),
                    },
                    ResponsePair {
                        id: example_pairs[1].id.clone(),
                        base: example_pairs[1].base.clone(),
                        value: fresh_example_pairs[1].value.clone(),
                        comparison: example_pairs[1].comparison.clone(),
                        created_at: example_pairs[1].created_at.clone(),
                        updated_at: fresh_example_pairs[1].updated_at.clone(),
                    },
                ],
                created_at: example_pair_groups[0].created_at.clone(),
                updated_at: response.pair_groups[0].updated_at.clone(),
            },
        );

        // pair_group2
        assert_eq!(
            response.pair_groups[1],
            ResponsePairGroup {
                id: example_pair_groups[1].id.clone(),
                is_pinned: false,
                pairs: vec![ResponsePair {
                    id: example_pairs[2].id.clone(),
                    base: example_pairs[2].base.clone(),
                    value: example_pairs[2].value.clone(),
                    comparison: example_pairs[2].comparison.clone(),
                    created_at: example_pairs[2].created_at.clone(),
                    updated_at: example_pairs[2].updated_at.clone(),
                },],
                created_at: example_pair_groups[1].created_at.clone(),
                updated_at: example_pair_groups[1].updated_at.clone(),
            },
        );
    }
}
