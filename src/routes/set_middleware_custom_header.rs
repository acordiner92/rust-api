use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let message = req
        .headers()
        .get("message")
        .ok_or(StatusCode::BAD_REQUEST)?;

    let message = message
        .to_str()
        .map_err(|_err| StatusCode::BAD_REQUEST)?
        .to_string();
    let extensions = req.extensions_mut();

    extensions.insert(HeaderMessage(message));

    Ok(next.run(req).await)
}
