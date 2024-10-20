use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{pair::Pair, pair_group::PairGroup},
    Error,
};

use super::interactor::Interactor;

pub trait UpdatePairGroupDataAccess {
    async fn update_pair_group(&mut self, pair_group: &PairGroup) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestPairGroup {
    pub id: String,
    pub is_pinned: bool,
    pub pairs: Vec<RequestPair>,
}

impl PartialEq for RequestPairGroup {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.is_pinned == other.is_pinned;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePairGroupRequest {
    pair_group: RequestPairGroup,
}

pub struct UpdatePairGroup<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<UpdatePairGroupRequest, ()> for UpdatePairGroup<DA>
where
    DA: UpdatePairGroupDataAccess,
{
    async fn perform(&mut self, request: UpdatePairGroupRequest) -> Result<(), Error> {
        /*
           TODO:
               - Make sure all pairs have the same base.
               - Make sure that there are no repeated pairs.
               - Make sure that the pair group exists.
               - Make sure that the it is not borrowing pairs from other pair group
               - Get the correct 'created_date's for the pair and pair group
        */
        let pair_group = PairGroup {
            id: request.pair_group.id,
            is_pinned: request.pair_group.is_pinned,
            pairs: request
                .pair_group
                .pairs
                .iter()
                .map(|p| Pair {
                    id: p.id.clone(),
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
        self.data_access.update_pair_group(&pair_group).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
