use sqlx;
use uuid;


enum AppError {
    Db(sqlx::Error),
    Parse(uuid::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Db(e)
    }
}

impl From<uuid::Error> for AppError {
    fn from(e: uuid::Error) -> Self {
        AppError::Parse(e)
    }
}
