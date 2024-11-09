use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entities::{pair::Pair, pair_group::PairGroup},
    Error,
};

use super::interactor::Interactor;

pub trait SavePairGroupDataAccess {
    async fn save_pair(&mut self, pair: &Pair) -> Result<(), Error>;
    async fn save_pair_group(&mut self, pair_group: &PairGroup) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestPair {
    pub value: f64,
    pub base: String,
    pub comparison: String,
}

impl PartialEq for RequestPair {
    fn eq(&self, other: &Self) -> bool {
        return self.base == other.base
            && self.value == other.value
            && self.comparison == other.comparison;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestPairGroup {
    pub is_pinned: bool,
    pub multiplier: f64,
    pub pairs: Vec<RequestPair>,
}

impl PartialEq for RequestPairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.pairs == other.pairs
            && self.is_pinned == other.is_pinned
            && self.multiplier == other.multiplier;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SavePairGroupRequest {
    pair_group: RequestPairGroup,
}

pub struct SavePairGroup<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<SavePairGroupRequest, ()> for SavePairGroup<DA>
where
    DA: SavePairGroupDataAccess,
{
    async fn perform(&mut self, request: SavePairGroupRequest) -> Result<(), Error> {
        validate_request(&request)?;
        let pair_group = PairGroup {
            id: Uuid::new_v4().to_string(),
            is_pinned: request.pair_group.is_pinned,
            multiplier: request.pair_group.multiplier,
            pairs: request
                .pair_group
                .pairs
                .iter()
                .map(|p| Pair {
                    id: Uuid::new_v4().to_string(),
                    base: p.base.clone(),
                    value: p.value.clone(),
                    comparison: p.comparison.clone(),
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                })
                .collect(),
            updated_at: Utc::now().to_rfc3339(),
            created_at: Utc::now().to_rfc3339(),
        };
        for pair in &pair_group.pairs {
            self.data_access.save_pair(pair).await?;
        }
        self.data_access.save_pair_group(&pair_group).await?;
        return Ok(());
    }
}

fn validate_request(request: &SavePairGroupRequest) -> Result<(), Error> {
    let pairs_len = request.pair_group.pairs.len();
    if pairs_len == 0 {
        return Err(Error {
            message: String::from("Cannot save a pair group that does not have pairs!"),
        });
    }
    if pairs_len > 1 {
        validate_request_pair_bases(&request)?;
        validate_request_duplicate_pairs(&request)?;
    }
    return Ok(());
}

fn validate_request_pair_bases(request: &SavePairGroupRequest) -> Result<(), Error> {
    let pairs_len = request.pair_group.pairs.len();
    let pair = &request.pair_group.pairs[0];
    for i in 1..pairs_len {
        let comparison_pair = &request.pair_group.pairs[i];
        if pair.base != comparison_pair.base {
            return Err(Error {
                message: String::from(
                    "Cannot save a pair group that contains pairs with different bases!",
                ),
            });
        }
    }
    return Ok(());
}

fn validate_request_duplicate_pairs(request: &SavePairGroupRequest) -> Result<(), Error> {
    let pairs_len = request.pair_group.pairs.len();
    for i in 0..pairs_len - 1 {
        let pair = &request.pair_group.pairs[i];
        for j in 1..pairs_len {
            let comparison_pair = &request.pair_group.pairs[j];
            if pair.comparison == comparison_pair.comparison {
                return Err(Error {
                    message: String::from(
                        "Cannot save a pair group that contains duplicate pairs!",
                    ),
                });
            }
        }
    }
    return Ok(());
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
