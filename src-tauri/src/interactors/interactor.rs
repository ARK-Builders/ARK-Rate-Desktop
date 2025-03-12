use crate::Error;

pub trait Interactor<R, S> {
    async fn perform(&mut self, request: R) -> Result<S, Error>;
}
