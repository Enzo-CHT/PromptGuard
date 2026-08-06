use std::{error::Error, fmt::Display, format, sync::Arc};

use axum::{Json, extract::State, http::StatusCode};
use helpers::text_segmenter::TextSegmenter;
use model::dto::{
    d2c::text_request_dto::TextRequestDto,
    drs::recognized_fragment_dto::RecognizedFragmentRequestBuilder,
};
use tracing::error;

use crate::service::{anonymization_service::AnonymizationService, drs_service::DrsService};

pub async fn is_alive() -> StatusCode {
    StatusCode::OK
}

pub async fn handle_text(
    State(d2c_service): State<Arc<AnonymizationService>>,
    State(drs_service): State<Arc<DrsService>>,
    Json(request): Json<TextRequestDto>,
) -> Result<Json<String>, (StatusCode, String)> {
    let TextRequestDto {
        text,
        filterlist,
        user_id,
    } = request;

    let text_segmenter = TextSegmenter::from_text(text.clone()).map_err(handle_error)?;

    let recognized_fragment_request = RecognizedFragmentRequestBuilder::default()
        .text_segmenter(text_segmenter)
        .filterlist(filterlist)
        .independant(true)
        .build()
        .map_err(handle_error)?;

    let recognized_fragment_list = drs_service
        .analyze(recognized_fragment_request)
        .await
        .map_err(handle_error)?;

    let result = d2c_service
        .mask_text(text, recognized_fragment_list, user_id)
        .await
        .map_err(handle_error)?;

    let result_text: String = result.text().clone();
    Ok(Json(result_text))
}

fn handle_error<E: Display>(e: E) -> (StatusCode, String) {
    error!("Erreur lors du traitement de la requête : {e}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Erreur lors du traitement de la requête: {e}"),
    )
}

#[cfg(test)]
mod tests {
    use std::println;

    use axum::body::Body;
    use axum::http::Request;
    use axum::routing::{Router, post};
    use helpers::filterlist::FilterList;
    use http_body_util::BodyExt;
    use model::dto::d2c::text_request_dto::TextRequestDtoBuilder;
    use regex::Regex;
    use repository::sled_mask_repository::SledMaskRepository;
    use serde_json::json;
    use tower::ServiceExt;

    use crate::{
        controller::app_state::AppState,
        service::mask_service::{self, MaskService},
    };

    use super::*;

    #[tokio::test]
    async fn test_handle_text() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mask_repository = Arc::new(SledMaskRepository::new(
            sled::open(temp_dir.path().to_str().unwrap()).unwrap(),
        ));
        let mask_service = MaskService::new(mask_repository);
        let app_state = AppState {
            d2c_service: Arc::new(AnonymizationService::new(mask_service)),
            drs_service: Arc::new(DrsService::new("http://localhost:9990")),
        };

        let app = Router::new()
            .route("/d2c/text", post(handle_text))
            .with_state(app_state);

        let filterlist = FilterList::new();
        let payload = json!(
            &TextRequestDtoBuilder::default()
                .text("Je m'appel Henry Cavill".into())
                .user_id("12345678910".into())
                .filterlist(filterlist)
                .build()
                .unwrap()
        );

        let request = Request::builder()
            .method("POST")
            .uri("/d2c/text")
            .header("Content-Type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        let pattern =
            Regex::new(r"^Je m'appel \[I-PER [0-9a-fA-F-]{5}\] \[I-PER [0-9a-fA-F-]{5}\]$")
                .unwrap();

        assert!(pattern.is_match(&body), "Format inattendu : {}", body);
    }
}
