use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    entities::{pair::Pair, pair_group::PairGroup, watchlist::Watchlist},
    utilities::coin_market::CoinMarket,
    Error,
};

use super::interactor::Interactor;

pub trait ViewWatchlistDataAccess {
    async fn update_pair(&mut self, pair: &Pair) -> Result<(), Error>;
    async fn find_watchlist(&mut self) -> Result<Option<Watchlist>, Error>;
    async fn save_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error>;
    async fn update_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error>;
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
pub struct ResponseRichPair {
    pub id: String,
    pub fluctuation: f64,
    pub value: f64,
    pub base: String,
    pub comparison: String,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for ResponseRichPair {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.fluctuation == other.fluctuation
            && self.base == other.base
            && self.value == other.value
            && self.comparison == other.comparison
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseWatchlist {
    pub id: String,
    pub pairs: Vec<ResponseRichPair>,
    pub created_at: String,
    pub updated_at: String,
}

impl PartialEq for ResponseWatchlist {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.pairs == other.pairs
            && self.created_at == other.created_at
            && self.updated_at == other.updated_at;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ViewWatchlistResponse {
    pub usd_pairs: Vec<ResponsePair>,
    pub watchlist: ResponseWatchlist,
}

pub struct ViewWatchlist<DA, CM> {
    pub data_access: DA,
    pub coin_market: CM,
}

impl<DA, CM> Interactor<(), ViewWatchlistResponse> for ViewWatchlist<DA, CM>
where
    DA: ViewWatchlistDataAccess,
    CM: CoinMarket,
{
    async fn perform(&mut self, _request: ()) -> Result<ViewWatchlistResponse, Error> {
        let usd_pairs = self.coin_market.fetch_usd_pairs().await?;
        if let Some(watchlist) = self.data_access.find_watchlist().await? {
            let fresh_watchlist = refresh_watchlist(&usd_pairs, &watchlist);
            for pair in &fresh_watchlist.pairs {
                self.data_access.update_pair(pair).await?;
            }
            self.data_access.update_watchlist(&fresh_watchlist).await?;
            return Ok(ViewWatchlistResponse {
                watchlist: create_response_watchlist(&watchlist, &fresh_watchlist),
                usd_pairs: usd_pairs
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
            });
        } else {
            let watchlist = Watchlist {
                id: Uuid::new_v4().to_string(),
                pairs: vec![],
                created_at: Utc::now().to_rfc3339(),
                updated_at: Utc::now().to_rfc3339(),
            };
            self.data_access.save_watchlist(&watchlist).await?;
            return Ok(ViewWatchlistResponse {
                watchlist: ResponseWatchlist {
                    id: watchlist.id.clone(),
                    pairs: vec![],
                    created_at: watchlist.created_at.clone(),
                    updated_at: watchlist.updated_at.clone(),
                },
                usd_pairs: usd_pairs
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
            });
        };
    }
}

fn refresh_watchlist(usd_pairs: &Vec<Pair>, watchlist: &Watchlist) -> Watchlist {
    let mut fresh_watchlist = Watchlist {
        id: watchlist.id.clone(),
        pairs: vec![],
        created_at: watchlist.created_at.clone(),
        updated_at: Utc::now().to_rfc3339(),
    };
    /* TODO:
        - check if pair is indeed an USD pair
        - improve handling of cases where the watchlist pair has not been found in USD pairs
    */
    for pair in &watchlist.pairs {
        let mut has_usd_pair = false;
        for usd_pair in usd_pairs {
            if usd_pair.comparison == pair.comparison {
                has_usd_pair = true;
                fresh_watchlist.pairs.push(Pair {
                    id: pair.id.clone(),
                    base: pair.base.clone(),
                    value: usd_pair.value.clone(),
                    comparison: pair.comparison.clone(),
                    created_at: pair.created_at.clone(),
                    updated_at: Utc::now().to_rfc3339(),
                });
                break;
            }
        }
        if !has_usd_pair {
            fresh_watchlist.pairs.push(pair.clone());
        }
    }
    return fresh_watchlist;
}

fn create_response_watchlist(
    watchlist: &Watchlist,
    fresh_watchlist: &Watchlist,
) -> ResponseWatchlist {
    let mut response_watchlist = ResponseWatchlist {
        id: fresh_watchlist.id.clone(),
        pairs: vec![],
        created_at: fresh_watchlist.created_at.clone(),
        updated_at: fresh_watchlist.updated_at.clone(),
    };
    for pair in &watchlist.pairs {
        for fresh_pair in &fresh_watchlist.pairs {
            if fresh_pair.id == pair.id {
                let difference = fresh_pair.value - pair.value;
                let fluctuation = difference / pair.value;
                response_watchlist.pairs.push(ResponseRichPair {
                    id: fresh_pair.id.clone(),
                    fluctuation,
                    base: fresh_pair.base.clone(),
                    value: fresh_pair.value.clone(),
                    comparison: fresh_pair.comparison.clone(),
                    created_at: fresh_pair.created_at.clone(),
                    updated_at: fresh_pair.updated_at.clone(),
                });
                break;
            }
        }
    }
    return response_watchlist;
}

#[cfg(test)]
mod test {
    // TODO: create unit tests for the interactor
}
