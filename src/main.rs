use std::error::Error;
use std::io::BufReader;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::fs;
use axum::{middleware, Router};
use tower_http::services::ServeDir;
use serde_json::Value;
use sqlx::{Pool, Postgres};
use dotenv;

mod db;
mod services;
mod model;
mod view;
mod routes;
mod core;
use routes::{
    default_middleware,
    handle_not_found,
    auth,
    controller,
};

#[derive(Clone)]
pub struct AppState {
    db_pool: Pool<Postgres>,
    navigation: Value,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("starting..");
    dotenv::dotenv().ok();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("env \'CARGO_MANIFEST_DIR\' must be set to the projects root directory, if not run by the cargo command.");
    println!("manifest_dir: {:?}", manifest_dir);

    let is_release = match std::env::var("RELEASE") {
        Ok(_) => true,
        Err(_) => false,
    };
    println!("is_release: {:?}", is_release);

    let navigation_file = fs::File::open(format!("{}/config/navigation.json", manifest_dir))?;
    let reader = BufReader::new(navigation_file);
    let navigation = serde_json::from_reader(reader)?;

    println!("starting db..");
    let db_pool = db::init().await?;
    println!("started db!");

    let app_state = AppState { db_pool, navigation };
    let app = Router::new()
        .merge(controller::product::routes())
        .merge(controller::shopping_list::routes())
        .merge(controller::social_timeline::routes())
        .merge(controller::static_page::routes())
        .merge(controller::user::routes())
        .merge(auth::routes())
        .fallback(handle_not_found)
        .layer(middleware::from_fn(auth::validate))
        .layer(middleware::from_fn(default_middleware))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(app_state);

    let port = match std::env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or(4000),
        Err(_) => 4000,
    };
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("server started on {:?}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
