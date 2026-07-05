/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

//! Todo backend with CSRF protection and CORS for React frontend.
//! Database: SQLite with connection pooling.

use axum::http::{HeaderValue, Method, header::HeaderName};
use axum_csrf::{CsrfConfig, CsrfLayer};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod auth;
mod database;
mod models;
mod routes;

const X_CSRF_TOKEN: HeaderName = HeaderName::from_static("x-csrf-token");

#[tokio::main]
async fn main() {
    let pool = database::create_pool()
        .await
        .expect("Failed to create database pool");

    let app = routes::create_router(pool)
        .layer(CsrfLayer::new(
            CsrfConfig::default().with_cookie_name("authenticity_token"),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::ACCEPT,
                    axum::http::header::ORIGIN,
                    X_CSRF_TOKEN,
                ])
                .allow_credentials(true),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
