use std::collections::HashMap;

use market::Market;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{coin::Coin, pair::Pair, pair_group::PairGroup},
    interactors::interactor::Error,
};

pub mod market;

