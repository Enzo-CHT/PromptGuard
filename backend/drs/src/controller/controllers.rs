use std::sync::{Arc, Mutex};

use crate::service::recognizer_service::RecognizerService;
use axum::{Json, extract::State, http::StatusCode};
use helpers::recognized_fragment_list::RecognizedFragmentList;
use model::dto::drs::recognized_fragment_dto::RecognizedFragmentRequest;

pub async fn analyze(
    State(service): State<Arc<Mutex<RecognizerService>>>,
    Json(request): Json<RecognizedFragmentRequest>,
) -> Result<Json<RecognizedFragmentList>, (StatusCode, String)> {
    // Récupération des fragments
    let fragment_list_result = tokio::task::spawn_blocking(move || {
        let service = service.lock().unwrap();
        service.analyze(request.text_segmenter, 0.1)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")))?
    .map_err(|e| (StatusCode::NOT_FOUND, "Entites not found ".to_string()))?;

    Ok(Json(fragment_list_result))
}

pub async fn is_alive() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, println};

    use super::*;
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::{get, post},
    };
    use helpers::text_segmenter::TextSegmenter;
    use http_body_util::BodyExt;
    use model::dto::drs::recognized_fragment_dto::RecognizedFragmentRequestBuilder;
    use serde_json::json;
    use tower::ServiceExt; // pour .oneshot() // pour .collect() sur le body de réponse

    #[tokio::test]
    async fn analyze_returns_ok_for_valid_request() {
        // 1. Construire un état de test
        let service =
            tokio::task::spawn_blocking(|| Arc::new(Mutex::new(RecognizerService::new().unwrap())))
                .await
                .unwrap();

        // 2. Construire un router isolé, juste pour ce endpoint
        let app = Router::new()
            .route("/drs/analyze", post(analyze))
            .with_state(service);

        // 3. Construire la requête

        let text_segmenter = TextSegmenter::from_text("Je m'appel Henry Cavill".into()).unwrap();
        let payload = json!(
            RecognizedFragmentRequestBuilder::default()
                .text_segmenter(text_segmenter)
                .independant(true)
                .build()
                .unwrap()
        );

        let request = Request::builder()
            .method("POST")
            .uri("/drs/analyze")
            .header("Content-Type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        // 4. Envoyer la requête à travers le router, sans réseau
        let response = app.oneshot(request).await.unwrap();

        // 5. Vérifier le résultat
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: RecognizedFragmentList = serde_json::from_slice(&body).unwrap();
        // assertions sur le contenu métier

        assert_eq!(body.list().len(), 2);
        assert_eq!(body.text(), "Henry Cavill");
    }

    #[tokio::test]
    async fn is_healthy() {
        let app = Router::new().route("/drs/isalive", get(is_alive));
        let request = Request::builder()
            .method("GET")
            .uri("/drs/isalive")
            .body(Body::from(""))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
