use chrono::Utc;
use serde::Serialize;

use crate::{
    entities::{pair::Pair, pair_group::PairGroup},
    utilities::coin_market::CoinMarket,
    Error,
};

use super::interactor::Interactor;

pub trait ViewPairGroupsDataAccess {
    async fn update_pair(&mut self, pair: &Pair) -> Result<(), Error>;
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
    pub multiplier: f64,
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
        let fresh_usd_pairs: Vec<Pair> = self.coin_market.retrieve_usd_pairs().await?;
        let stored_pair_groups: Vec<PairGroup> = self.data_access.fetch_pair_groups().await?;
        for stored_pair_group in &stored_pair_groups {
            if stored_pair_group.is_pinned {
                let fresh_pair_group = refresh_pair_group(&fresh_usd_pairs, stored_pair_group)?;
                update_pair_group(&mut self.data_access, &fresh_pair_group).await?;
                pair_groups.push(fresh_pair_group);
            } else {
                pair_groups.push(stored_pair_group.clone())
            }
        }
        pair_groups.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        pair_groups
            .iter_mut()
            .for_each(|pg| pg.pairs.sort_by(|a, b| a.created_at.cmp(&b.created_at)));
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
                    multiplier: pg.multiplier,
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
                        .collect::<Vec<ResponsePair>>(),
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
        multiplier: usd_pair_group.multiplier,
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
        multiplier: non_usd_pair_group.multiplier,
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
                    value: equivalent_usd_value / fresh_usd_pair.value,
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

async fn update_pair_group(
    data_access: &mut impl ViewPairGroupsDataAccess,
    pair_group: &PairGroup,
) -> Result<(), Error> {
    for pair in &pair_group.pairs {
        data_access.update_pair(pair).await?;
    }
    data_access.update_pair_group(pair_group).await?;
    return Ok(());
}

#[cfg(test)]
mod test {
    // TODO: create unit tests for the interactor
}
