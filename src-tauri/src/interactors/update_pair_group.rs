use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entities::{pair::Pair, pair_group::PairGroup},
    Error,
};

use super::interactor::Interactor;

pub trait UpdatePairGroupDataAccess {
    async fn find_pair(&mut self, id: &str) -> Result<Option<Pair>, Error>;
    async fn delete_pair(&mut self, id: &str) -> Result<(), Error>;
    async fn save_pair(&mut self, pair: &Pair) -> Result<(), Error>;
    async fn update_pair(&mut self, pair: &Pair) -> Result<(), Error>;
    async fn find_pair_group(&mut self, id: &str) -> Result<Option<PairGroup>, Error>;
    async fn update_pair_group(&mut self, pair_group: &PairGroup) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestPair {
    pub id: String,
    pub value: f64,
    pub base: String,
    pub comparison: String,
}

impl PartialEq for RequestPair {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.base == other.base
            && self.value == other.value
            && self.comparison == other.comparison;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestPairGroup {
    pub id: String,
    pub is_pinned: bool,
    pub multiplier: f64,
    pub pairs: Vec<RequestPair>,
}

impl PartialEq for RequestPairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.is_pinned == other.is_pinned
            && self.multiplier == other.multiplier;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdatePairGroupRequest {
    pub pair_group: RequestPairGroup,
}

pub struct UpdatePairGroup<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<UpdatePairGroupRequest, ()> for UpdatePairGroup<DA>
where
    DA: UpdatePairGroupDataAccess,
{
    async fn perform(&mut self, request: UpdatePairGroupRequest) -> Result<(), Error> {
        validate_request(&request)?;
        let maybe_pair_group = self
            .data_access
            .find_pair_group(&request.pair_group.id)
            .await?;
        if maybe_pair_group.is_none() {
            return Err(Error {
                message: String::from("Pair group to update does not exist!"),
            });
        }
        let pair_group = maybe_pair_group.unwrap();
        let mut updated_pair_group = PairGroup {
            id: request.pair_group.id,
            is_pinned: request.pair_group.is_pinned,
            multiplier: request.pair_group.multiplier,
            pairs: vec![],
            created_at: pair_group.created_at.clone(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let mut added_pairs: Vec<Pair> = vec![];
        let mut updated_pairs: Vec<Pair> = vec![];
        let mut removed_pairs: Vec<Pair> = vec![];

        for request_pair in &request.pair_group.pairs {
            let maybe_pair = self.data_access.find_pair(&request_pair.id).await?;
            if maybe_pair.is_none() {
                let pair = Pair {
                    id: Uuid::new_v4().to_string(),
                    base: request_pair.base.clone(),
                    value: request_pair.value.clone(),
                    comparison: request_pair.comparison.clone(),
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                };
                added_pairs.push(pair.clone());
                updated_pair_group.pairs.push(pair);
                continue;
            }
            let pair = maybe_pair.unwrap();
            if !pair_group.pairs.contains(&pair) {
                return Err(Error {
                    message: String::from("Cannot borrow pairs from other pair groups!"),
                });
            }
            let updated_pair = Pair {
                id: request_pair.id.clone(),
                base: request_pair.base.clone(),
                value: request_pair.value.clone(),
                comparison: request_pair.comparison.clone(),
                created_at: pair_group.created_at.clone(),
                updated_at: Utc::now().to_rfc3339(),
            };
            updated_pairs.push(updated_pair.clone());
            updated_pair_group.pairs.push(updated_pair);
        }

        for pair in &pair_group.pairs {
            let mut is_removed = true;
            for request_pair in &request.pair_group.pairs {
                if request_pair.id == pair.id {
                    is_removed = false;
                    break;
                }
            }
            if is_removed {
                removed_pairs.push(pair.clone());
            }
        }

        for pair in &added_pairs {
            self.data_access.save_pair(pair).await?;
        }
        for pair in &updated_pairs {
            self.data_access.update_pair(pair).await?;
        }
        for pair in &removed_pairs {
            self.data_access.delete_pair(&pair.id).await?;
        }

        self.data_access
            .update_pair_group(&updated_pair_group)
            .await?;
        return Ok(());
    }
}

fn validate_request(request: &UpdatePairGroupRequest) -> Result<(), Error> {
    let pairs_len = request.pair_group.pairs.len();
    if pairs_len == 0 {
        return Err(Error {
            message: String::from("Cannot update a pair group that does not have pairs!"),
        });
    }
    if pairs_len > 1 {
        validate_request_pair_bases(&request)?;
        validate_request_duplicate_pairs(&request)?;
    }
    return Ok(());
}

fn validate_request_pair_bases(request: &UpdatePairGroupRequest) -> Result<(), Error> {
    let pairs_len = request.pair_group.pairs.len();
    let pair = &request.pair_group.pairs[0];
    for i in 1..pairs_len {
        let comparison_pair = &request.pair_group.pairs[i];
        if pair.base != comparison_pair.base {
            return Err(Error {
                message: String::from(
                    "Cannot update a pair group that contains pairs with different bases!",
                ),
            });
        }
    }
    return Ok(());
}

fn validate_request_duplicate_pairs(request: &UpdatePairGroupRequest) -> Result<(), Error> {
    let pairs_len = request.pair_group.pairs.len();
    for i in 0..pairs_len - 1 {
        let pair = &request.pair_group.pairs[i];
        for j in 1..pairs_len {
            let comparison_pair = &request.pair_group.pairs[j];
            if pair.comparison == comparison_pair.comparison {
                return Err(Error {
                    message: String::from(
                        "Cannot update a pair group that contains duplicate pairs!",
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
