// use crate::{
//     entities::{coin::Coin, create_entity::create_entity, pair::Pair, pair_group::PairGroup},
//     utilities::market::Market,
// };

// use super::interactor::{Error, Interactor};

// pub trait SavePairGroupDataAccess {
//     async fn find_coin(&self, id: &str) -> Result<Option<Coin>, Error>;
//     async fn save_pair_group(&self, pair_group: &PairGroup) -> Result<(), Error>;
// }

// pub struct SavePairGroupRequest {
//     base_id: String,
//     comparison_ids: Vec<String>,
// }

// pub struct SavePairGroup<T, U> {
//     data_access: T,
//     market: U
// }

// impl<T, U> Interactor<SavePairGroupRequest, ()> for SavePairGroup<T, U>
// where
//     T: SavePairGroupDataAccess,
//     U: Market
// {
//     async fn perform(&self, request: SavePairGroupRequest) -> Result<(), Error> {
//         let maybe_base: Option<Coin> = self.data_access.find_coin(&request.base_id).await?;
//         if maybe_base.is_none() {
//             return Err(Error {
//                 message: String::from("Unsupported coin code!"),
//             });
//         }
//         let base: Coin = maybe_base.unwrap();
//         let base_pairs = self.market.retrieve_updated_pairs_by_base(&base).await?;
//         for comparison_id in &request.comparison_ids {
//             let maybe_comparison: Option<Coin> = self.data_access.find_coin(&comparison_id).await?;
//             if maybe_comparison.is_none() {
//                 return Err(Error {
//                     message: String::from("One of the comparisons was not found!"),
//                 });
//             }
//             let comparison: Coin = maybe_comparison.unwrap();
//             for base_pair in &base_pairs {
//                 if base_pair.comparison.code == comparison.code {

//                 }
//             }
//             let pair: Pair = Pair {
//                 entity: create_entity(),
//                 base: base.clone(),
//                 comparison: comparison.clone(),
//                 value
//             }
//         }
//         return Ok(());
//     }
// }
