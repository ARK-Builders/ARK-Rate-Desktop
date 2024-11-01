use serde::Serialize;

use crate::Error;

use super::interactor::Interactor;

pub trait ViewPortfoliosDataAccess {
    // TODO: add methods or delete it
}

#[derive(Clone, Debug, Serialize)]
pub struct ViewPortfoliosResponse {
    // TODO: add data or delete it
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
