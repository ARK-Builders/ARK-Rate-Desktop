use crate::{entities::pair::Pair, interactors::interactor::Error};

pub trait Market {
    async fn retrieve_updated_pairs_by_base_coin_code(&self, code: &str) -> Result<Vec<Pair>, Error>;
}
