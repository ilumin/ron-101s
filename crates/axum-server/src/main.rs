use std::net::SocketAddr;

use axum::{Router, routing::get, Extension, Json};
use db::User;
use errors::CustomError;

mod config;
mod errors;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let pool = db::create_pool(&config.database_url);

    let app = Router::new()
        .route("/", get(users))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    print!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;
    let users = db::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    Ok(Json(users))
}
