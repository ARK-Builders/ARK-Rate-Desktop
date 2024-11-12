use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Error {
    pub message: String,
}
