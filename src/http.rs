use std::any::Any;

use serde::Serialize;

pub struct RequestBuilder(http::request::Builder);

impl RequestBuilder {
    pub fn new() -> Self {
        Self(http::request::Builder::new())
    }

    pub fn contains_header(&self, key: impl http::header::AsHeaderName) -> bool {
        self.0
            .headers_ref()
            .is_some_and(|headers| headers.contains_key(key))
    }

    pub fn json<T: Serialize + ?Sized>(
        mut self,
        json: &T,
    ) -> crate::Result<http::Request<reqwest::Body>> {
        match serde_json::to_vec(json) {
            Ok(body) => {
                if !self.contains_header(http::header::CONTENT_TYPE) {
                    self.0 = self.0.header(
                        http::header::CONTENT_TYPE,
                        http::header::HeaderValue::from_static("application/json"),
                    );
                }
                Ok(self.0.body(body.into())?)
            }
            Err(err) => Err(crate::Error::HttpError(Box::new(err))),
        }
    }
}

impl RequestBuilder {
    pub fn method<T>(mut self, method: T) -> Self
    where
        T: TryInto<http::Method>,
        <T as TryInto<http::Method>>::Error: Into<http::Error>,
    {
        self.0 = self.0.method(method);
        self
    }

    pub fn method_ref(&self) -> Option<&http::Method> {
        self.0.method_ref()
    }

    pub fn uri<T>(mut self, uri: T) -> Self
    where
        T: TryInto<http::Uri>,
        <T as TryInto<http::Uri>>::Error: Into<http::Error>,
    {
        self.0 = self.0.uri(uri);
        self
    }

    pub fn uri_ref(&self) -> Option<&http::Uri> {
        self.0.uri_ref()
    }

    pub fn version(mut self, version: http::Version) -> Self {
        self.0 = self.0.version(version);
        self
    }

    pub fn version_ref(&self) -> Option<&http::Version> {
        self.0.version_ref()
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: TryInto<http::HeaderName>,
        <K as TryInto<http::HeaderName>>::Error: Into<http::Error>,
        V: TryInto<http::HeaderValue>,
        <V as TryInto<http::HeaderValue>>::Error: Into<http::Error>,
    {
        self.0 = self.0.header(key, value);
        self
    }

    pub fn headers_ref(&self) -> Option<&http::HeaderMap<http::HeaderValue>> {
        self.0.headers_ref()
    }

    pub fn headers_mut(&mut self) -> Option<&mut http::HeaderMap<http::HeaderValue>> {
        self.0.headers_mut()
    }

    pub fn extension<T>(mut self, extension: T) -> Self
    where
        T: Clone + Any + Send + Sync + 'static,
    {
        self.0 = self.0.extension(extension);
        self
    }

    pub fn extensions_ref(&self) -> Option<&http::Extensions> {
        self.0.extensions_ref()
    }

    pub fn extensions_mut(&mut self) -> Option<&mut http::Extensions> {
        self.0.extensions_mut()
    }

    pub fn body<T>(self, body: T) -> Result<http::Request<T>, crate::Error> {
        Ok(self.0.body(body)?)
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
