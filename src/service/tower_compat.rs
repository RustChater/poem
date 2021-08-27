use std::{error::Error as StdError, fmt::Display, future::Future, pin::Pin};

use hyper::body::HttpBody;
use tower::{buffer::Buffer, make::Shared, MakeService, Service, ServiceExt};

use crate::{http, http::StatusCode, Endpoint, Error, Request, Response};

type BoxRequest = http::Request<hyper::Body>;

type BoxResponse = http::Response<hyper::Body>;

type BoxError = Box<dyn StdError + Send + Sync + 'static>;

type BoxServiceFuture = Pin<Box<dyn Future<Output = Result<BoxResponse, BoxError>>>>;

type BoxService = Box<
    dyn Service<BoxRequest, Response = BoxResponse, Error = BoxError, Future = BoxServiceFuture>,
>;

// #[cfg(feature = "tower-compat")]
// #[cfg_attr(docsrs, doc(cfg(feature = "tower-compat")))]
// pub struct TowerCompat {
//     inner: Shared<Buffer<BoxService, http::Request<Box<dyn HttpBody>>>>,
// }
//
// #[async_trait::async_trait]
// impl Endpoint for TowerCompat
// where
//     T: Service<http::Request<ReqBody>, Response = http::Response<RepBody>> +
// Send + Sync + 'static,     T::Future: Send,
//     T::Error: StdError + Send + Sync + 'static,
//     ReqBody: HttpBody + Send + Sync + 'static,
//     RepBody: HttpBody + Send + Sync + 'static,
// {
//     type Output = Response;
//
//     async fn call(&self, req: Request) -> Self::Output {
//         let mut into_service = MakeService::<T,
// _>::into_service(self.inner.clone());         let mut inner: Buffer<T,
// http::Request<ReqBody>> = into_service.call(()).await.unwrap();         //
// inner.call(req.         // into_hyper_request()); let _ = inner
//         //     .ready()
//         //     .await
//         //     .map_err(|_| Error::new(StatusCode::INTERNAL_SERVER_ERROR));
//         //
//         // // a(Request::default().into_hyper_request());
//         // inner.call(req.into_hyper_request()).await;
//         todo!()
//         // match inner.call(req.into_hyper_request()).await {
//         //     Ok(resp) => Response::from_http_response(resp),
//         //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into(),
//         // }
//     }
// }
//
// #[cfg(feature = "tower-compat")]
// #[cfg_attr(docsrs, doc(cfg(feature = "tower-compat")))]
// trait TowerCompatExt {
//     fn compat<ReqBody, RepBody>(self) -> TowerCompat
//     where
//         Self: Service<http::Request<ReqBody>, Response =
// http::Response<RepBody>>             + Send
//             + Sync
//             + 'static,
//         Self::Future: Send,
//         Self::Error: StdError + Send + Sync + 'static,
//         ReqBody: HttpBody + Send + Sync + 'static,
//         RepBody: HttpBody + Send + Sync + 'static,
//     {
//         TowerCompat {
//             inner: Shared::new(Buffer::new(self, 32)),
//         }
//     }
// }

struct ServiceWrapper<S, ReqBody, RepBody>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<RepBody>> + Send + Sync + 'static,
    S::Future: Send,
    S::Error: StdError + Send + Sync + 'static,
    ReqBody: HttpBody + Send + Sync + 'static,
    RepBody: HttpBody + Send + Sync + 'static,
{
    inner: S,
}
