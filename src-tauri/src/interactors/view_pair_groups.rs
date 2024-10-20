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
    pub usd_pairs: Vec<ResponsePair>,
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
        let fresh_usd_pairs: Vec<Pair> = self.coin_market.retrieve_pairs_by_base("USD").await?;
        let stored_pair_groups: Vec<PairGroup> = self.data_access.fetch_pair_groups().await?;
        for stored_pair_group in &stored_pair_groups {
            if stored_pair_group.is_pinned {
                let fresh_pair_group = refresh_pair_group(&fresh_usd_pairs, stored_pair_group)?;
                self.data_access
                    .update_pair_group(&fresh_pair_group)
                    .await?;
                pair_groups.push(fresh_pair_group);
            } else {
                pair_groups.push(stored_pair_group.clone())
            }
        }
        return Ok(ViewPairGroupsResponse {
            usd_pairs: fresh_usd_pairs
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

fn refresh_pair_group(
    fresh_usd_pairs: &Vec<Pair>,
    pair_group: &PairGroup,
) -> Result<PairGroup, Error> {
    /*
        TODO:
            - got to make sure the new pair group has the same size as the old one
    */
    if pair_group.pairs[0].base == "USD" {
        return refresh_usd_pair_group(fresh_usd_pairs, pair_group);
    } else {
        return refresh_non_usd_pair_group(fresh_usd_pairs, pair_group);
    }
}

fn refresh_usd_pair_group(
    fresh_usd_pairs: &Vec<Pair>,
    usd_pair_group: &PairGroup,
) -> Result<PairGroup, Error> {
    let mut fresh_usd_pair_group = PairGroup {
        id: usd_pair_group.id.clone(),
        pairs: vec![],
        is_pinned: usd_pair_group.is_pinned,
        created_at: usd_pair_group.created_at.clone(),
        updated_at: Utc::now().to_rfc3339(),
    };
    for usd_pair in &usd_pair_group.pairs {
        for fresh_usd_pair in fresh_usd_pairs {
            if usd_pair.comparison == fresh_usd_pair.comparison {
                fresh_usd_pair_group.pairs.push(Pair {
                    id: usd_pair.id.clone(),
                    base: String::from("USD"),
                    value: fresh_usd_pair.value.clone(),
                    comparison: usd_pair.comparison.clone(),
                    created_at: usd_pair.created_at.clone(),
                    updated_at: Utc::now().to_rfc3339(),
                });
                break;
            };
        }
    }
    return Ok(fresh_usd_pair_group);
}

fn refresh_non_usd_pair_group(
    fresh_usd_pairs: &Vec<Pair>,
    non_usd_pair_group: &PairGroup,
) -> Result<PairGroup, Error> {
    let mut fresh_non_usd_pair_group = PairGroup {
        id: non_usd_pair_group.id.clone(),
        pairs: vec![],
        is_pinned: non_usd_pair_group.is_pinned,
        created_at: non_usd_pair_group.created_at.clone(),
        updated_at: Utc::now().to_rfc3339(),
    };
    let equivalent_usd_value =
        get_equivalent_usd_value(fresh_usd_pairs, &non_usd_pair_group.pairs[0].base)?;
    for non_usd_pair in &non_usd_pair_group.pairs {
        for fresh_usd_pair in fresh_usd_pairs {
            if non_usd_pair.base == fresh_usd_pair.comparison {
                fresh_non_usd_pair_group.pairs.push(Pair {
                    id: non_usd_pair.id.clone(),
                    base: non_usd_pair.base.clone(),
                    value: equivalent_usd_value,
                    comparison: non_usd_pair.comparison.clone(),
                    created_at: non_usd_pair.created_at.clone(),
                    updated_at: Utc::now().to_rfc3339(),
                });
                break;
            } else if non_usd_pair.comparison == fresh_usd_pair.comparison {
                fresh_non_usd_pair_group.pairs.push(Pair {
                    id: non_usd_pair.id.clone(),
                    base: non_usd_pair.base.clone(),
                    value: equivalent_usd_value * fresh_usd_pair.value,
                    comparison: non_usd_pair.comparison.clone(),
                    created_at: non_usd_pair.created_at.clone(),
                    updated_at: Utc::now().to_rfc3339(),
                });
                break;
            }
        }
    }
    return Ok(fresh_non_usd_pair_group);
}

fn get_equivalent_usd_value(usd_pairs: &Vec<Pair>, target_base: &str) -> Result<f64, Error> {
    for usd_pair in usd_pairs {
        if usd_pair.comparison == target_base {
            return Ok(1.0 / usd_pair.value);
        }
    }
    return Err(Error {
        message: String::from("Could not find the equivalent USD value for the target base!"),
    });
}

#[cfg(test)]
mod test {
    use super::*;

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

    fn round_to_decimal_places(x: f64, decimal_places: i8) -> f64 {
        return (x * 10f64.powi(decimal_places.into())).round() / 10f64.powi(decimal_places.into());
    }

    #[tokio::test]
    async fn test_fresh_usd_pairs() {
        /*
           This is a scenario to assert that the interactor is returning the fresh usd pairs.
        */

        let fresh_usd_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p4".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T06:00:00+00:00"),
                updated_at: String::from("2024-01-01T07:00:00+00:00"),
            },
            Pair {
                id: "p5".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: String::from("2024-01-01T08:00:00+00:00"),
                updated_at: String::from("2024-01-01T09:00:00+00:00"),
            },
            Pair {
                id: "p6".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T11:00:00+00:00"),
            },
        ];

        let data_access = ViewPairGroupsDataAccessMock {
            fetch_pair_groups_result: Ok(vec![]),
            update_pair_group_result: Ok(()),
        };
        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_usd_example_pairs.clone()),
        };
        let mut interactor = ViewPairGroups {
            data_access,
            coin_market,
        };

        let response = interactor.perform(()).await.unwrap();

        assert_eq!(
            response.usd_pairs,
            fresh_usd_example_pairs
                .iter()
                .map(|p| ResponsePair {
                    id: p.id.clone(),
                    base: p.base.clone(),
                    value: p.value.clone(),
                    comparison: p.comparison.clone(),
                    created_at: p.created_at.clone(),
                    updated_at: p.updated_at.clone(),
                })
                .collect::<Vec<ResponsePair>>()
        );
    }

    #[tokio::test]
    async fn test_pinned_pair_group_with_usd_base() {
        /*  This is a pinned pair group scenario, where the pair group has USD as its base.
            Main expectations for the 'example_pair_group':
                - It should receive new pairs, based on 'fresh_usd_example_pairs'.
                - The new pairs should have the same ID's as their matches from the old pair that has the same base and comparison coins.
                - 'created_at' for the pair group and contained pairs should remain the same.
                - 'updated_at' for the pair group should be updated.
        */

        let example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 1.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T00:00:00+00:00"),
                updated_at: String::from("2024-01-01T01:00:00+00:00"),
            },
            Pair {
                id: "p2".to_string(),
                value: 2.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: String::from("2024-01-01T02:00:00+00:00"),
                updated_at: String::from("2024-01-01T03:00:00+00:00"),
            },
            Pair {
                id: "p3".to_string(),
                value: 3.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T04:00:00+00:00"),
                updated_at: String::from("2024-01-01T05:00:00+00:00"),
            },
        ];

        let fresh_usd_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p4".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T06:00:00+00:00"),
                updated_at: String::from("2024-01-01T07:00:00+00:00"),
            },
            Pair {
                id: "p5".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: String::from("2024-01-01T08:00:00+00:00"),
                updated_at: String::from("2024-01-01T09:00:00+00:00"),
            },
            Pair {
                id: "p6".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T11:00:00+00:00"),
            },
        ];

        let example_pair_group = PairGroup {
            id: "pg1".to_string(),
            is_pinned: true,
            pairs: example_pairs.clone(),
            created_at: String::from("2024-01-01T00:00:00+00:00"),
            updated_at: String::from("2024-01-01T05:00:00+00:00"),
        };

        let data_access = ViewPairGroupsDataAccessMock {
            fetch_pair_groups_result: Ok(vec![example_pair_group.clone()]),
            update_pair_group_result: Ok(()),
        };
        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_usd_example_pairs.clone()),
        };
        let mut interactor = ViewPairGroups {
            data_access,
            coin_market,
        };

        let response = interactor.perform(()).await.unwrap();

        assert_eq!(response.pair_groups.len(), 1);

        // example pair group
        let response_pair_group = &response.pair_groups[0];
        assert_eq!(response_pair_group.id, "pg1");
        assert_eq!(response_pair_group.pairs.len(), 3);
        assert_eq!(response_pair_group.is_pinned, true);
        assert_eq!(response_pair_group.created_at, "2024-01-01T00:00:00+00:00");
        assert_ne!(response_pair_group.updated_at, "2024-01-01T05:00:00+00:00");

        // example pair 0
        assert_eq!(response_pair_group.pairs[0].id, "p1");
        assert_eq!(response_pair_group.pairs[0].value, 4.0);
        assert_eq!(response_pair_group.pairs[0].base, "USD");
        assert_eq!(response_pair_group.pairs[0].comparison, "BTC");
        assert_eq!(
            response_pair_group.pairs[0].created_at,
            "2024-01-01T00:00:00+00:00"
        );
        assert_ne!(
            response_pair_group.pairs[0].updated_at,
            "2024-01-01T01:00:00+00:00"
        );

        // example pair 1
        assert_eq!(response_pair_group.pairs[1].id, "p2");
        assert_eq!(response_pair_group.pairs[1].value, 5.0);
        assert_eq!(response_pair_group.pairs[1].base, "USD");
        assert_eq!(response_pair_group.pairs[1].comparison, "ETH");
        assert_eq!(
            response_pair_group.pairs[1].created_at,
            "2024-01-01T02:00:00+00:00"
        );
        assert_ne!(
            response_pair_group.pairs[1].updated_at,
            "2024-01-01T03:00:00+00:00"
        );

        // example pair 2
        assert_eq!(response_pair_group.pairs[2].id, "p3");
        assert_eq!(response_pair_group.pairs[2].value, 6.0);
        assert_eq!(response_pair_group.pairs[2].base, "USD");
        assert_eq!(response_pair_group.pairs[2].comparison, "BRL");
        assert_eq!(
            response_pair_group.pairs[2].created_at,
            "2024-01-01T04:00:00+00:00"
        );
        assert_ne!(
            response_pair_group.pairs[2].updated_at,
            "2024-01-01T05:00:00+00:00"
        );
    }

    #[tokio::test]
    async fn test_unpinned_pair_group() {
        /*  This is a unpinned pair group scenario.
            Main expectations for the 'example_pair_group':
                - It should just receive the old pairs.
                - Both 'created_at' and 'updated_at' for the pair group and contained pairs should remain the same.
        */

        let example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 1.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T00:00:00+00:00"),
                updated_at: String::from("2024-01-01T01:00:00+00:00"),
            },
            Pair {
                id: "p2".to_string(),
                value: 2.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: String::from("2024-01-01T02:00:00+00:00"),
                updated_at: String::from("2024-01-01T03:00:00+00:00"),
            },
            Pair {
                id: "p3".to_string(),
                value: 3.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T04:00:00+00:00"),
                updated_at: String::from("2024-01-01T05:00:00+00:00"),
            },
        ];

        let fresh_usd_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p4".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T06:00:00+00:00"),
                updated_at: String::from("2024-01-01T07:00:00+00:00"),
            },
            Pair {
                id: "p5".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: String::from("2024-01-01T08:00:00+00:00"),
                updated_at: String::from("2024-01-01T09:00:00+00:00"),
            },
            Pair {
                id: "p6".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T11:00:00+00:00"),
            },
        ];

        let example_pair_group = PairGroup {
            id: "pg1".to_string(),
            is_pinned: false,
            pairs: example_pairs.clone(),
            created_at: String::from("2024-01-01T00:00:00+00:00"),
            updated_at: String::from("2024-01-01T05:00:00+00:00"),
        };

        let data_access = ViewPairGroupsDataAccessMock {
            fetch_pair_groups_result: Ok(vec![example_pair_group.clone()]),
            update_pair_group_result: Ok(()),
        };
        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_usd_example_pairs.clone()),
        };
        let mut interactor = ViewPairGroups {
            data_access,
            coin_market,
        };

        let response = interactor.perform(()).await.unwrap();

        assert_eq!(response.pair_groups.len(), 1);

        // example pair group
        let response_pair_group = &response.pair_groups[0];
        assert_eq!(response_pair_group.id, "pg1");
        assert_eq!(response_pair_group.pairs.len(), 3);
        assert_eq!(response_pair_group.is_pinned, false);
        assert_eq!(response_pair_group.created_at, "2024-01-01T00:00:00+00:00");
        assert_eq!(response_pair_group.updated_at, "2024-01-01T05:00:00+00:00");

        // example pair 0
        assert_eq!(response_pair_group.pairs[0].id, "p1");
        assert_eq!(response_pair_group.pairs[0].value, 1.0);
        assert_eq!(response_pair_group.pairs[0].base, "USD");
        assert_eq!(response_pair_group.pairs[0].comparison, "BTC");
        assert_eq!(
            response_pair_group.pairs[0].created_at,
            "2024-01-01T00:00:00+00:00"
        );
        assert_eq!(
            response_pair_group.pairs[0].updated_at,
            "2024-01-01T01:00:00+00:00"
        );

        // example pair 1
        assert_eq!(response_pair_group.pairs[1].id, "p2");
        assert_eq!(response_pair_group.pairs[1].value, 2.0);
        assert_eq!(response_pair_group.pairs[1].base, "USD");
        assert_eq!(response_pair_group.pairs[1].comparison, "ETH");
        assert_eq!(
            response_pair_group.pairs[1].created_at,
            "2024-01-01T02:00:00+00:00"
        );
        assert_eq!(
            response_pair_group.pairs[1].updated_at,
            "2024-01-01T03:00:00+00:00"
        );

        // example pair 2
        assert_eq!(response_pair_group.pairs[2].id, "p3");
        assert_eq!(response_pair_group.pairs[2].value, 3.0);
        assert_eq!(response_pair_group.pairs[2].base, "USD");
        assert_eq!(response_pair_group.pairs[2].comparison, "BRL");
        assert_eq!(
            response_pair_group.pairs[2].created_at,
            "2024-01-01T04:00:00+00:00"
        );
        assert_eq!(
            response_pair_group.pairs[2].updated_at,
            "2024-01-01T05:00:00+00:00"
        );
    }

    #[tokio::test]
    async fn test_pinned_pair_group_with_different_base() {
        /*  This is a pinned pair group scenario, where the pair group has a base different than USD.
            Main expectations for the 'example_pair_group':
                - It should receive fresh usd pairs that have equivalent calculated values according to their base conversions.
        */

        let example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p1".to_string(),
                value: 1.0,
                base: "BRL".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T00:00:00+00:00"),
                updated_at: String::from("2024-01-01T01:00:00+00:00"),
            },
            Pair {
                id: "p2".to_string(),
                value: 2.0,
                base: "BRL".to_string(),
                comparison: "ETH".to_string(),
                created_at: String::from("2024-01-01T02:00:00+00:00"),
                updated_at: String::from("2024-01-01T03:00:00+00:00"),
            },
            Pair {
                id: "p3".to_string(),
                value: 3.0,
                base: "BRL".to_string(),
                comparison: "USD".to_string(),
                created_at: String::from("2024-01-01T04:00:00+00:00"),
                updated_at: String::from("2024-01-01T05:00:00+00:00"),
            },
        ];

        let fresh_usd_example_pairs: Vec<Pair> = vec![
            Pair {
                id: "p4".to_string(),
                value: 4.0,
                base: "USD".to_string(),
                comparison: "BTC".to_string(),
                created_at: String::from("2024-01-01T06:00:00+00:00"),
                updated_at: String::from("2024-01-01T07:00:00+00:00"),
            },
            Pair {
                id: "p5".to_string(),
                value: 5.0,
                base: "USD".to_string(),
                comparison: "ETH".to_string(),
                created_at: String::from("2024-01-01T08:00:00+00:00"),
                updated_at: String::from("2024-01-01T09:00:00+00:00"),
            },
            Pair {
                id: "p6".to_string(),
                value: 6.0,
                base: "USD".to_string(),
                comparison: "BRL".to_string(),
                created_at: String::from("2024-01-01T10:00:00+00:00"),
                updated_at: String::from("2024-01-01T11:00:00+00:00"),
            },
        ];

        let example_pair_group = PairGroup {
            id: "pg1".to_string(),
            is_pinned: true,
            pairs: example_pairs.clone(),
            created_at: String::from("2024-01-01T00:00:00+00:00"),
            updated_at: String::from("2024-01-01T05:00:00+00:00"),
        };

        let data_access = ViewPairGroupsDataAccessMock {
            fetch_pair_groups_result: Ok(vec![example_pair_group.clone()]),
            update_pair_group_result: Ok(()),
        };
        let coin_market = CoinMarketMock {
            retrieve_pairs_by_base_result: Ok(fresh_usd_example_pairs.clone()),
        };
        let mut interactor = ViewPairGroups {
            data_access,
            coin_market,
        };

        let response = interactor.perform(()).await.unwrap();

        assert_eq!(response.pair_groups.len(), 1);

        // example pair group
        let response_pair_group = &response.pair_groups[0];
        assert_eq!(response_pair_group.id, "pg1");
        assert_eq!(response_pair_group.pairs.len(), 3);
        assert_eq!(response_pair_group.is_pinned, true);
        assert_eq!(response_pair_group.created_at, "2024-01-01T00:00:00+00:00");
        assert_ne!(response_pair_group.updated_at, "2024-01-01T05:00:00+00:00");

        // example pair 0
        assert_eq!(response_pair_group.pairs[0].id, "p1");
        assert_eq!(
            round_to_decimal_places(response_pair_group.pairs[0].value, 15),
            round_to_decimal_places(4.0 / 6.0, 15)
        );
        assert_eq!(response_pair_group.pairs[0].base, "BRL");
        assert_eq!(response_pair_group.pairs[0].comparison, "BTC");
        assert_eq!(
            response_pair_group.pairs[0].created_at,
            "2024-01-01T00:00:00+00:00"
        );
        assert_ne!(
            response_pair_group.pairs[0].updated_at,
            "2024-01-01T01:00:00+00:00"
        );

        // example pair 1
        assert_eq!(response_pair_group.pairs[1].id, "p2");
        assert_eq!(
            round_to_decimal_places(response_pair_group.pairs[1].value, 15),
            round_to_decimal_places(5.0 / 6.0, 15)
        );
        assert_eq!(response_pair_group.pairs[1].base, "BRL");
        assert_eq!(response_pair_group.pairs[1].comparison, "ETH");
        assert_eq!(
            response_pair_group.pairs[1].created_at,
            "2024-01-01T02:00:00+00:00"
        );
        assert_ne!(
            response_pair_group.pairs[1].updated_at,
            "2024-01-01T03:00:00+00:00"
        );

        // example pair 2
        assert_eq!(response_pair_group.pairs[2].id, "p3");
        assert_eq!(
            round_to_decimal_places(response_pair_group.pairs[2].value, 15),
            round_to_decimal_places(1.0 / 6.0, 15)
        );
        assert_eq!(response_pair_group.pairs[2].base, "BRL");
        assert_eq!(response_pair_group.pairs[2].comparison, "USD");
        assert_eq!(
            response_pair_group.pairs[2].created_at,
            "2024-01-01T04:00:00+00:00"
        );
        assert_ne!(
            response_pair_group.pairs[2].updated_at,
            "2024-01-01T05:00:00+00:00"
        );
    }
}
