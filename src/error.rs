#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Native kiwi error: {0}")]
    Native(String),
}
