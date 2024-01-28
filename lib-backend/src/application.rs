use std::{error::Error, fmt::Debug, marker::PhantomData};
use axum::{extract::FromRef, response::IntoResponse, Router};
use tracing::info;

pub struct Application<S,E> {
    phantom: PhantomData<E>,
    listener: tokio::net::TcpListener,
    router: Router<S>,
}

impl<S,E> Application<S,E>
where 
    E: Debug + Clone + From<std::io::Error> + Error + IntoResponse,
    S: Clone + Sync + Send + FromRef<S> + 'static,
{
    pub async fn new(port: u16) -> std::result::Result<Self, E> {
        let addr = format!("0.0.0.0:{}", port);
        info!("Binding the socket on port: {port}.");
        Ok(Self { listener: tokio::net::TcpListener::bind(addr).await?, phantom: PhantomData, router: Router::new() })
    }
}

impl<S,E> Application<S,E>
where
    E: Debug + Clone + From<std::io::Error> + Error + IntoResponse,
    S: Clone + FromRef<S> + Sync + Send + 'static,
{
    // FIXME: This if stupid, find the way to remove 'state: S' from params.
    pub async fn run(self, router: Router<S>, state: S) -> std::result::Result<(), E> {
        let r = self.router
            .merge(router)
            .with_state(state);
        
        info!("Backend started.");
        Ok(axum::serve(self.listener, r).await?)
    }
}