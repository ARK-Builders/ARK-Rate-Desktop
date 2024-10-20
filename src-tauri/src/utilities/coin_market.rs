use crate::{entities::pair::Pair, Error};

pub trait CoinMarket {
    async fn retrieve_usd_pairs(&mut self) -> Result<Vec<Pair>, Error>;
}
