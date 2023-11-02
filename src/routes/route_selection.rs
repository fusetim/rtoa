use axum::{
    extract,
    response::{IntoResponse},
};

use crate::views::{HtmlTemplate, RouteSelection};

pub async fn route_selection(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = RouteSelection {};
    HtmlTemplate(template)
}