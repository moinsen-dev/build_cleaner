use axum::{
    body::Body,
    http::{header, Response, StatusCode},
    routing::{get, post},
    Router,
};
use rust_embed::RustEmbed;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::cli::Args;

use super::api;
use super::state::AppState;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

/// Start the web server
pub async fn start_server(args: Args) -> anyhow::Result<()> {
    let port = args.port;
    let no_open = args.no_open;

    let state = Arc::new(AppState::new(args));

    let app = Router::new()
        // API routes
        .route("/api/status", get(api::get_status))
        .route("/api/scan", get(api::trigger_scan))
        .route("/api/scan/progress", get(api::scan_progress))
        .route("/api/results", get(api::get_results))
        .route("/api/clean", post(api::execute_clean))
        .route("/api/generate-script", post(api::generate_script))
        // Static files
        .route("/", get(serve_index))
        .route("/*path", get(serve_asset))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    println!("\n  Build Cleaner Web UI");
    println!("  ────────────────────────────────────────");
    println!("  Local:   http://{}", addr);
    println!("  ────────────────────────────────────────\n");

    // Auto-open browser unless --no-open
    if !no_open {
        let url = format!("http://{}", addr);
        if let Err(e) = open::that(&url) {
            eprintln!("  Could not open browser: {}", e);
            println!("  Open {} in your browser\n", url);
        }
    }

    println!("  Press Ctrl+C to stop the server\n");

    axum::serve(listener, app).await?;

    Ok(())
}

/// Serve index.html
async fn serve_index() -> Response<Body> {
    serve_embedded_file("index.html")
}

/// Serve other static files
async fn serve_asset(axum::extract::Path(path): axum::extract::Path<String>) -> Response<Body> {
    serve_embedded_file(&path)
}

/// Serve an embedded file
fn serve_embedded_file(path: &str) -> Response<Body> {
    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path)
                .first_or_octet_stream()
                .to_string();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime)
                .body(Body::from(content.data.to_vec()))
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from("Not Found"))
            .unwrap(),
    }
}
