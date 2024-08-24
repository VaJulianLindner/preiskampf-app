use std::error::Error;
use std::io::BufReader;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::fs;
use handlebars::Handlebars;
use axum::{middleware, Router, routing};
use routing::{get, post};
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

use core::pagination::prev_page_if_last_item;
use core::query_params::preserve_query_params;
use routes::{
    print_timestamp_middleware,
    handle_not_found,
    button_type,
    check_auth_required,
    hide_if_authenticated,
    format_price,
    xor,
    sub,
    add,
    concat,
    humanize_utc_time,
    emoji_list,
    templates::page_template,
    details::detail_template,
    auth::{validate, authorize, register, logout},
    controller,
};

#[derive(Clone)]
pub struct AppState {
    engine: Handlebars<'static>,
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

    let mut engine = Handlebars::new();
    engine.set_strict_mode(false);
    engine.set_dev_mode(!is_release);
    engine.register_templates_directory(".hbs", format!("{}/templates_old", manifest_dir))?;
    engine.register_helper("button_type", Box::new(button_type));
    engine.register_helper("check_auth_required", Box::new(check_auth_required));
    engine.register_helper("hide_if_authenticated", Box::new(hide_if_authenticated));
    engine.register_helper("format_price", Box::new(format_price));
    engine.register_helper("xor", Box::new(xor));
    engine.register_helper("sub", Box::new(sub));
    engine.register_helper("add", Box::new(add));
    engine.register_helper("concat", Box::new(concat));
    engine.register_helper("preserve_query_params", Box::new(preserve_query_params));
    engine.register_helper("humanize_utc_time", Box::new(humanize_utc_time));
    engine.register_helper("emoji_list", Box::new(emoji_list));
    engine.register_helper("prev_page_if_last_item", Box::new(prev_page_if_last_item));

    let file = fs::File::open(format!("{}/config/navigation.json", manifest_dir))?;
    let reader = BufReader::new(file);
    let navigation = serde_json::from_reader(reader)?;

    println!("starting db..");
    let db_pool = db::init().await?;
    println!("started db!");

    let app_state = AppState { engine, db_pool, navigation };
    let app = Router::new()
        .merge(controller::product::routes())
        .merge(controller::shopping_list::routes())
        .merge(controller::social_timeline::routes())
        .merge(controller::user::routes())
        .route("/:template_name/:resource_id", post(detail_template))
        .route("/:template_name/:resource_id", get(detail_template))
        .route("/:template_name", post(page_template))
        .route("/:template_name", get(page_template))
        .route("/", get(page_template))
        .route("/authorize", post(authorize))
        .route("/register", post(register))
        .route("/logout", post(logout))
        // TODO need error handler i guess
        .fallback(handle_not_found)
        .layer(middleware::from_fn(validate))
        .layer(middleware::from_fn(print_timestamp_middleware))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(app_state);

    let port = match std::env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => 4000,
    };
    println!("server using port {:?}", port);
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("server started on {:?}!", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
