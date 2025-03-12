use serde::Deserialize;

use crate::{entities::watchlist::Watchlist, Error};

use super::interactor::Interactor;

pub trait DeleteWatchlistPairDataAccess {
    async fn delete_pair(&mut self, id: &str) -> Result<(), Error>;
    async fn get_watchlist(&mut self) -> Result<Watchlist, Error>;
    async fn update_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestPair {
    pub id: String,
}

impl PartialEq for RequestPair {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteWatchlistPairRequest {
    pub pair: RequestPair,
}

pub struct DeleteWatchlistPair<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<DeleteWatchlistPairRequest, ()> for DeleteWatchlistPair<DA>
where
    DA: DeleteWatchlistPairDataAccess,
{
    async fn perform(&mut self, request: DeleteWatchlistPairRequest) -> Result<(), Error> {
        /* TODO:
            - Check for non-existing pair
            - Check if pair is really from watchlist (to avoid deleting a pair from a pair group for example)
        */
        let mut watchlist = self.data_access.get_watchlist().await?;
        watchlist.pairs.retain(|p| p.id != request.pair.id);
        self.data_access.delete_pair(&request.pair.id).await?;
        self.data_access.update_watchlist(&watchlist).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
