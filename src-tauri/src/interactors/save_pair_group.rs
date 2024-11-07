use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entities::{pair::Pair, pair_group::PairGroup},
    Error,
};

use super::interactor::Interactor;

pub trait SavePairGroupDataAccess {
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
        /*
           TODO:
               - Make sure all pairs have the same base.
               - Make sure that there are no repeated pairs.
        */
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
        self.data_access.save_pair_group(&pair_group).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
