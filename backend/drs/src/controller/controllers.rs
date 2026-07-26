use std::sync::{Arc, Mutex};

use crate::service::{self, recognizer_service::RecognizerService};
use axum::{Json, extract::State, http::request};
use model::dto::ars::recognized_fragment_dto::RecognizedFragmentRequest;

pub async fn analyze(
    State(service): State<Arc<Mutex<RecognizerService>>>,
    Json(request): Json<RecognizedFragmentRequest>,
) -> String {
    let entities = tokio::task::spawn_blocking(move || {
        let service = service.lock().unwrap();
        service.analyze(request.text_segmenter, 0.1)
    })
    .await
    .unwrap();

    format!("{:?}", entities)
}

#[cfg(test)]
mod tests {

    use super::*;
    use helpers::filterlist::FilterList;
    use helpers::text_segmenter::TextSegmenter;

    #[test]
    fn print_request_json_shape() {
        let request = RecognizedFragmentRequest {
            text_segmenter: TextSegmenter::from_text(String::from("Test")).unwrap(),
            entities: vec!["TEST".into()],
            entities_coordinates: vec![(1, 2)],
            filterlist: FilterList::default(),
            independant: true,
        };

        let json = serde_json::to_string_pretty(&request).unwrap();
        println!("{}", json);
    }
}
