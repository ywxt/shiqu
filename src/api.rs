mod gemini;

pub trait Api {
    fn from_token(token: impl Into<String>, model: impl Into<String>) -> Self;

    fn create_request(&self, message: &str) -> crate::Result<http::Request<reqwest::Body>>;

    fn streaming_response(
        &self,
        response: reqwest::Response,
    ) -> reqwest::Result<impl tokio_stream::Stream<Item = reqwest::Result<String>>>;
}

#[derive(Debug, Clone)]
pub struct ModelApi<M> {
    model: M,
    client: reqwest::Client,
}

impl<M: Api> ModelApi<M> {
    fn from_token(token: &str, model: &str) -> Self {
        Self {
            model: M::from_token(token, model),
            client: reqwest::Client::new(),
        }
    }

    async fn send_messsage(
        &self,
        message: &str,
    ) -> crate::Result<impl tokio_stream::Stream<Item = reqwest::Result<String>>> {
        let request = self.model.create_request(message)?;
        let reqwest_request = reqwest::Request::try_from(request)?;
        let response = self.client.execute(reqwest_request).await?;
        Ok(self.model.streaming_response(response)?)
    }
}
