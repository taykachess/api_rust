use axum::{http::StatusCode, response::Html, response::IntoResponse, Json};
use thiserror::Error;

#[derive(Error, Debug)]
enum CustomError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Database error: {0}")]
    DBError(String),
}

// Implement `IntoResponse` so that `CustomError` can be converted to an HTTP response.
impl IntoResponse for CustomError {
    type Body = Html<String>;
    type BodyError = std::convert::Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let status_code = match &self {
            CustomError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            CustomError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = format!("{}", self);
        Html(body)
            .into_response()
            .map(move |response| {
                response.map(|mut response| {
                    *response.status_mut() = status_code;
                    response
                })
            })
            .unwrap_or_else(|_| axum::http::Response::new(StatusCode::INTERNAL_SERVER_ERROR))
    }
}

// Example usage
async fn handle_post(data: Json<MyData>) -> Result<Json<MyResponse>, CustomError> {
    if data.is_invalid() {
        return Err(CustomError::InvalidInput("Invalid data".to_string()));
    }
    let result = my_database_operation()
        .await
        .map_err(|e| CustomError::DBError(e.to_string()))?;
    Ok(Json(result))
}
