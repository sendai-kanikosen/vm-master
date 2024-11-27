// TODO: エラーをさらに細かく階層化する

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ValidationError(#[from] pub anyhow::Error);

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct DatabaseError(#[from] pub anyhow::Error);

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ProxmoxApiError(#[from] pub anyhow::Error);

pub type ValidationResult<T> = Result<T, ValidationError>;
pub type DatabaseResult<T> = Result<T, DatabaseError>;
pub type ProxmoxApiResult<T> = Result<T, ProxmoxApiError>;
