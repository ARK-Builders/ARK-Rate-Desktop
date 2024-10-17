use crate::{
    entities::{coin::Coin, create_entity::create_entity, pair::Pair, pair_group::PairGroup},
    utilities::market::Market,
};

use super::interactor::{Error, Interactor};

pub trait ViewPairGroupsDataAccess {
    async fn fetch_pair_groups(&self) -> Result<Vec<PairGroup>, Error>;
}

#[derive(Debug)]
pub struct ViewPairGroupsResponse {
    pub pinned_pair_groups: Vec<PairGroup>,
    pub calculated_pair_groups: Vec<PairGroup>,
    pub frequent_pair_groups: Vec<PairGroup>,
}

pub struct ViewPairGroups<T, U> {
    data_access: T,
    market: U,
}

impl<T, U> Interactor<(), ViewPairGroupsResponse> for ViewPairGroups<T, U>
where
    T: ViewPairGroupsDataAccess,
    U: Market,
{
    async fn perform(&self, _request: ()) -> Result<ViewPairGroupsResponse, Error> {
        let mut pinned_pair_groups: Vec<PairGroup> = vec![];
        let mut calculated_pair_groups: Vec<PairGroup> = vec![];
        let frequent_pair_groups: Vec<PairGroup> = vec![]; // TODO: implement the logic to get this
        let updated_pairs: Vec<Pair> = self
            .market
            .retrieve_updated_pairs_by_base_coin_code("USD")
            .await?;
        let pair_groups: Vec<PairGroup> = self.data_access.fetch_pair_groups().await?;
        for pair_group in &pair_groups {
            if pair_group.is_pinned {
                let mut updated_pair_group: PairGroup = PairGroup {
                    entity: pair_group.entity.clone(),
                    is_pinned: true,
                    pairs: vec![],
                };
                for pair in &pair_group.pairs {
                    for updated_pair in &updated_pairs {
                        let has_same_base: bool = pair.base.code == updated_pair.base.code;
                        let has_same_comparison: bool =
                            pair.comparison.code == updated_pair.comparison.code;
                        let has_base_eq_comparison: bool =
                            pair.base.code == updated_pair.comparison.code;
                        let has_comparison_eq_base: bool =
                            pair.comparison.code == updated_pair.base.code;
                        if has_same_base && has_same_comparison {
                            updated_pair_group.pairs.push(updated_pair.clone());
                        } else if has_base_eq_comparison && has_comparison_eq_base {
                            let inverted_value: f64 = 1.0 / updated_pair.value;
                            let inverted_updated_pair: Pair = Pair {
                                entity: create_entity(),
                                base: pair.base.clone(),
                                comparison: pair.comparison.clone(),
                                value: inverted_value,
                            };
                            updated_pair_group.pairs.push(inverted_updated_pair)
                        }
                    }
                }
                pinned_pair_groups.push(updated_pair_group)
            } else {
                calculated_pair_groups.push(pair_group.clone());
            }
        }
        return Ok(ViewPairGroupsResponse {
            pinned_pair_groups,
            calculated_pair_groups,
            frequent_pair_groups,
        });
    }
}
