use crate::{entities::pair::Pair, Error};

pub trait CoinMarket {
    async fn retrieve_pairs_by_base(&mut self, base: &str) -> Result<Vec<Pair>, Error>;
}
