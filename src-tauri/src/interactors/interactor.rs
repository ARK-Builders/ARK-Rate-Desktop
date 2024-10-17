#[derive(Debug)]
pub struct Error {
    pub message: String,
}

pub trait Interactor<R, S> {
    async fn perform(&self, request: R) -> Result<S, Error>;
}
