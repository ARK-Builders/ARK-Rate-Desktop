use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entities::{pair::Pair, watchlist::Watchlist},
    utilities::coin_market::CoinMarket,
    Error,
};

use super::interactor::Interactor;

pub trait StoreWatchlistCoinsDataAccess {
    async fn save_pair(&mut self, pair: &Pair) -> Result<(), Error>;
    async fn get_watchlist(&mut self) -> Result<Watchlist, Error>;
    async fn update_watchlist(&mut self, watchlist: &Watchlist) -> Result<(), Error>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoreWatchlistCoinsRequest {
    pub coins: Vec<String>,
}

pub struct StoreWatchlistCoins<DA, CM> {
    pub data_access: DA,
    pub coin_market: CM,
}

impl<DA, CM> Interactor<StoreWatchlistCoinsRequest, ()> for StoreWatchlistCoins<DA, CM>
where
    DA: StoreWatchlistCoinsDataAccess,
    CM: CoinMarket,
{
    async fn perform(&mut self, request: StoreWatchlistCoinsRequest) -> Result<(), Error> {
        /* TODO:
            - Check for repeated coins
        */
        let mut watchlist = self.data_access.get_watchlist().await?;
        let usd_pairs = self.coin_market.fetch_usd_pairs().await?;
        for coin in &request.coins {
            for usd_pair in &usd_pairs {
                if usd_pair.comparison.eq(coin) {
                    let pair = Pair {
                        id: Uuid::new_v4().to_string(),
                        base: String::from("USD"),
                        comparison: coin.clone(),
                        value: usd_pair.value.clone(),
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                    };
                    self.data_access.save_pair(&pair).await?;
                    watchlist.pairs.push(pair);
                    break;
                }
            }
        }
        self.data_access.update_watchlist(&watchlist).await?;
        return Ok(());
    }
}

#[cfg(test)]
mod test {
    // TODO: write tests for this `impl``
}
