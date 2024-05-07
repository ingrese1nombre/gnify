use std::{borrow::Borrow, net::TcpListener};

use axum::{routing::get, Json, Router};
use smol::{Async, Executor};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::application::{self, AppState};

pub mod auth {
    
}

pub async fn run<'ex>(ex: impl Borrow<Executor<'ex>> + Clone + Send + 'ex) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(
            |_| "axum_login=debug,tower_sessions=debug,sqlx=warn,tower_http=debug".into(),
        )))
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;
    let url = env!("DATABASE_URL");
    let state = AppState::init(url).await.expect("Couldn't start server");

    let app = Router::new().route("/", get(handler)).with_state(state);
    let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 3000)).unwrap();
    println!("listening on http://{}", listener.get_ref().local_addr().unwrap());
    smol_axum::serve(ex, listener, app).await?;
    Ok(())
}

async fn handler() -> Json<&'static phf::Map<&'static str, &'static [&'static str]>> {
    Json(&application::PRIVILEGES)
}