use crate::{entities::pair::Pair, Error};

pub trait CoinMarket {
    async fn fetch_usd_pairs(&mut self) -> Result<Vec<Pair>, Error>;
}
