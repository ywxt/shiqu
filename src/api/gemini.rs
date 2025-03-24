use tokio_stream::StreamExt;

use crate::http::RequestBuilder;

use super::Api;

#[derive(Debug)]
pub struct GeminiApi {
    token: String,
    model: String,
}

impl Api for GeminiApi {
    fn from_token(token: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            model: model.into(),
        }
    }

    fn create_request(&self, message: &str) -> crate::Result<http::Request<reqwest::Body>> {
        RequestBuilder::new()
        .method("POST")
        .uri(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{MODEL_ID}:streamGenerateContent?alt=sse&key={GEMINI_API_KEY}",
            MODEL_ID = self.model,
            GEMINI_API_KEY = self.token
        )).json(&serde_json::json!({
            "contents": [
                {
                    "parts": [{"text": message}],
                    "role": "user"
                }
            ]
        }))
    }

    fn streaming_response(
        &self,
        response: reqwest::Response,
    ) -> reqwest::Result<impl tokio_stream::Stream<Item = reqwest::Result<String>>> {
        let response = response.error_for_status()?;
        Ok(response
            .bytes_stream()
            .map(|item| item.map(|bytes| String::from_utf8_lossy(&bytes).to_string())))
    }
}
