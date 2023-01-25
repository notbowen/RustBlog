#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to get ctx")]
    CtxFail,

    #[error("Value not of type '{0}'")]
    ValueNotOfType(&'static str),

    #[error("Property '{0}' not found")]
    PropertyNotFound(String),

    #[error("Fail to create. Cause: {0}")]
    StoreFailToCreate(String),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
