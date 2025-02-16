use tracing::{Instrument, Level};

use super::Middleware;
use crate::{Endpoint, IntoResponse, Request, Response};

/// A middleware for tracing requests and responses
#[cfg_attr(docsrs, doc(cfg(feature = "tracing")))]
#[derive(Default)]
pub struct Tracing;

impl Tracing {
    /// Create new `Tracing` middleware.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl<E: Endpoint> Middleware<E> for Tracing {
    type Output = TracingImpl<E>;

    fn transform(self, ep: E) -> Self::Output {
        TracingImpl { inner: ep }
    }
}

#[doc(hidden)]
pub struct TracingImpl<E> {
    inner: E,
}

#[async_trait::async_trait]
impl<E: Endpoint> Endpoint for TracingImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Self::Output {
        let span = tracing::span!(
            Level::INFO,
            "handle request",
            method = %req.method(),
            path = %req.uri(),
        );

        let fut = self.inner.call(req);
        async move {
            let resp = fut.await.into_response();

            if resp.is_success() {
                ::tracing::info!(status = %resp.status(), "send response");
            } else {
                ::tracing::error!(status = %resp.status(), "an error occurred");
            }

            resp
        }
        .instrument(span)
        .await
    }
}
