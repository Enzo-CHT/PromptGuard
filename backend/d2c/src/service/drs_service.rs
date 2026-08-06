use std::format;

use helpers::recognized_fragment_list::RecognizedFragmentList;
use model::dto::drs::recognized_fragment_dto::RecognizedFragmentRequest;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct DrsService {
    client: reqwest::Client,
    base_url: String,
}

impl DrsService {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into(),
        }
    }
    pub async fn analyze(
        &self,
        recognized_fragment_request: RecognizedFragmentRequest,
    ) -> anyhow::Result<RecognizedFragmentList> {
        let payload = json!(recognized_fragment_request);

        let request = self
            .client
            .post(format!("{}/drs/analyze", self.base_url))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        let response = request.error_for_status()?;
        let result: RecognizedFragmentList = response.json().await?;

        Ok(result)
    }
}
