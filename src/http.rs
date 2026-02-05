use futures::future::BoxFuture;
use futures::FutureExt;
use gpui::http_client::{AsyncBody, HttpClient, Request, Response};
use std::sync::Arc;

#[cfg(feature = "http")]
pub struct SimpleHttpClient {
    client: isahc::HttpClient,
    user_agent: gpui::http_client::http::HeaderValue,
}

#[cfg(feature = "http")]
impl SimpleHttpClient {
    pub fn new(user_agent: &str) -> Result<Arc<Self>, isahc::Error> {
        let client = isahc::HttpClient::builder()
            .default_header("User-Agent", user_agent)
            .build()?;

        let user_agent_header = gpui::http_client::http::HeaderValue::from_str(user_agent)
            .unwrap_or_else(|_| gpui::http_client::http::HeaderValue::from_static("adabraka-ui"));

        Ok(Arc::new(Self {
            client,
            user_agent: user_agent_header,
        }))
    }

    pub fn with_default_user_agent() -> Result<Arc<Self>, isahc::Error> {
        Self::new("adabraka-ui/0.2.3")
    }
}

#[cfg(feature = "http")]
impl HttpClient for SimpleHttpClient {
    fn type_name(&self) -> &'static str {
        "SimpleHttpClient"
    }

    fn user_agent(&self) -> Option<&gpui::http_client::http::HeaderValue> {
        Some(&self.user_agent)
    }

    fn proxy(&self) -> Option<&gpui::http_client::Url> {
        None
    }

    fn send(
        &self,
        req: Request<AsyncBody>,
    ) -> BoxFuture<'static, gpui::http_client::Result<Response<AsyncBody>>> {
        let client = self.client.clone();
        let (parts, _body) = req.into_parts();

        async move {
            let method_str = parts.method.as_str();
            let uri_str = parts.uri.to_string();

            let mut request_builder = isahc::Request::builder().method(method_str).uri(&uri_str);

            for (key, value) in parts.headers.iter() {
                if let Ok(value_str) = value.to_str() {
                    request_builder = request_builder.header(key.as_str(), value_str);
                }
            }

            let isahc_request = request_builder
                .body(())
                .map_err(|e| gpui::http_client::anyhow!("Failed to build request: {}", e))?;

            let mut response = client
                .send_async(isahc_request)
                .await
                .map_err(|e| gpui::http_client::anyhow!("HTTP request failed: {}", e))?;

            let status = response.status();
            let headers = response.headers().clone();

            use isahc::AsyncReadResponseExt;
            let bytes = response
                .bytes()
                .await
                .map_err(|e| gpui::http_client::anyhow!("Failed to read response body: {}", e))?;

            let mut builder = gpui::http_client::http::Response::builder().status(
                gpui::http_client::http::StatusCode::from_u16(status.as_u16())
                    .map_err(|e| gpui::http_client::anyhow!("Invalid status code: {}", e))?,
            );

            for (key, value) in headers.iter() {
                builder = builder.header(key.as_str(), value.as_bytes());
            }

            let async_body = AsyncBody::from_bytes(bytes::Bytes::from(bytes));
            let response = builder
                .body(async_body)
                .map_err(|e| gpui::http_client::anyhow!("Failed to build response: {}", e))?;

            Ok(response)
        }
        .boxed()
    }
}

pub fn init_http(cx: &mut gpui::App) {
    #[cfg(feature = "http")]
    {
        if let Ok(client) = SimpleHttpClient::with_default_user_agent() {
            cx.set_http_client(client);
        }
    }
    #[cfg(not(feature = "http"))]
    {
        let _ = cx;
    }
}

pub fn init_http_with_user_agent(cx: &mut gpui::App, user_agent: &str) {
    #[cfg(feature = "http")]
    {
        if let Ok(client) = SimpleHttpClient::new(user_agent) {
            cx.set_http_client(client);
        }
    }
    #[cfg(not(feature = "http"))]
    {
        let _ = cx;
        let _ = user_agent;
    }
}
