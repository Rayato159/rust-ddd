use axum::{async_trait, http::StatusCode, response::IntoResponse, Json};

use super::models::Item;

#[async_trait]
pub trait Controller {
    async fn listing() -> impl IntoResponse;
}

struct ControllerImpl;

impl ControllerImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Controller for ControllerImpl {
    async fn listing() -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(vec![Item {
                id: 1,
                name: "Sword".into(),
                description: "Sword of the darkness.".into(),
                picture: Some("https://picture.com/swordofdarkness.png".into()),
                price: 150.,
            }])
            .into_response(),
        )
    }
}
