use serde::Serialize;

use crate::Error;

use super::interactor::Interactor;

pub trait ViewPortfoliosDataAccess {
    // TODO: add methods or delete it
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseTag {
    pub id: String,
    pub name: String,
}

impl PartialEq for ResponseTag {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.name == other.name;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseAsset {
    pub id: String,
    pub coin: String,
    pub quantity: f64,
}

impl PartialEq for ResponseAsset {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.coin == other.coin && self.quantity == other.quantity;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponsePortfolio {
    pub usd_value: f64,
    pub tag: ResponseTag,
    pub asset: ResponseAsset,
}

impl PartialEq for ResponsePortfolio {
    fn eq(&self, other: &Self) -> bool {
        return self.usd_value == other.usd_value
            && self.tag == other.tag
            && self.asset == other.asset;
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ViewPortfoliosResponse {
    portfolios: Vec<ResponsePortfolio>,
}

pub struct ViewPortfolios<DA> {
    pub data_access: DA,
}

impl<DA> Interactor<(), ViewPortfoliosResponse> for ViewPortfolios<DA>
where
    DA: ViewPortfoliosDataAccess,
{
    async fn perform(&mut self, _request: ()) -> Result<ViewPortfoliosResponse, Error> {
        // TODO: implement it
        return Err(Error {
            message: String::from("Not implemented!"),
        });
    }
}

#[cfg(test)]
mod test {
    // TODO: create tests for the interactor
}
