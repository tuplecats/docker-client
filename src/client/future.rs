
use crate::client::response::DockerResponse;

use hyper::rt::Future;
use tokio::prelude::stream::Concat2;
use tokio::prelude::Async;

pub struct DockerFuture {
    pub status: hyper::StatusCode,
    pub body: Concat2<hyper::Body>,
}

impl Future for DockerFuture {
    type Item = DockerResponse;
    type Error = hyper::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.body.poll().map(|s| {
            match s {
                Async::NotReady => Ok(Async::<Self::Item>::NotReady),
                Async::Ready(s) => {
                    Ok(Async::Ready(DockerResponse{
                        status: self.status.as_u16(),
                        body: std::str::from_utf8(&s).unwrap_or("").to_string(),
                    }))
                },
            }
        }).unwrap()
    }
}