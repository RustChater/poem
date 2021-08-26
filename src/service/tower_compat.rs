use std::marker::PhantomData;

use hyper::body::HttpBody;
use tower_service::Service;
use tower_util::ServiceExt;

use crate::{http, Endpoint, Request, Response};

#[cfg(feature = "tower-compat")]
#[cfg_attr(docsrs, doc(cfg(feature = "tower-compat")))]
pub struct TowerCompat<T, ReqBody, RepBody> {
    inner: T,
    _mark1: PhantomData<ReqBody>,
    _mark2: PhantomData<RepBody>,
}

#[async_trait::async_trait]
impl<T, ReqBody, RepBody> Endpoint for TowerCompat<T, ReqBody, RepBody>
where
    T: Service<http::Request<ReqBody>, Response = http::Response<RepBody>>
        + Send
        + Sync
        + Sized
        + 'static,
    ReqBody: HttpBody + Send + Sync + 'static,
    RepBody: HttpBody + Send + Sync + 'static,
{
    type Output = Response;

    async fn call(&self, req: Request) -> Self::Output {
        // let r = self.inner.ready_and().await;
        todo!()
    }
}

#[cfg(feature = "tower-compat")]
#[cfg_attr(docsrs, doc(cfg(feature = "tower-compat")))]
pub trait TowerCompatExt {
    fn compat<ReqBody, RepBody>(self) -> TowerCompat<Self, ReqBody, RepBody>
    where
        Self: Service<http::Request<ReqBody>, Response = http::Response<RepBody>>
            + Send
            + Sized
            + 'static,
        ReqBody: HttpBody + Send + Sync + 'static,
        RepBody: HttpBody + Send + Sync + 'static,
    {
        TowerCompat {
            inner: self,
            _mark1: PhantomData,
            _mark2: PhantomData,
        }
    }
}
