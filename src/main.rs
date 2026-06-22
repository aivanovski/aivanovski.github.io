extern crate core;

mod error;
mod export;
mod model;
mod posts;
mod views;

use crate::model::CliArguments;
use crate::posts::PostsRepository;
use crate::views::about::render_about;
use crate::views::home::render_home;
use crate::views::post::render_post;

use crate::export::export_static_site;
use axum::extract::State;
use axum::{
    Router,
    extract::Path as AxumPath,
    http::{HeaderValue, StatusCode, header::CACHE_CONTROL},
    response::Html,
    routing::get,
};
use clap::Parser;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::{services::ServeDir, set_header::SetResponseHeader};

const PORT: u16 = 3000;
const POSTS_DIRECTORY: &str = "posts";
const STATIC_DIRECTORY: &str = "static";

#[derive(Clone)]
pub struct AppState {
    pub posts_repository: Arc<PostsRepository>,
}

#[tokio::main]
async fn main() {
    let cli = CliArguments::parse();

    if let Some(output) = cli.output {
        export_static_site(&output).expect("Unable to export static site");
        println!("Static site exported to {}", output.display());
        return;
    }

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", PORT))
        .await
        .unwrap_or_else(|_| panic!("Unable to bind server to port {PORT}"));

    println!("Listening on http://{}/", listener.local_addr().unwrap());

    let app = create_app();

    axum::serve(listener, app)
        .await
        .expect("Server startup failed");
}

fn create_app() -> Router {
    let static_files = SetResponseHeader::overriding(
        ServeDir::new("./static"),
        CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );

    let app_state = AppState {
        posts_repository: create_posts_repository(),
    };

    Router::new()
        .route("/", get(home))
        .route("/index", get(home))
        .route("/home", get(home))
        .route("/posts", get(home))
        .route("/about", get(about))
        .route("/posts/{path}", get(post))
        .nest_service("/static", static_files)
        .with_state(app_state)
}

fn create_posts_repository() -> Arc<PostsRepository> {
    Arc::new(PostsRepository::new(PathBuf::from(POSTS_DIRECTORY)))
}

fn internal_server_error(error: impl Display) -> StatusCode {
    eprintln!("{error}");
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn home(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let posts = state
        .posts_repository
        .list_posts()
        .map_err(internal_server_error)?;

    Ok(Html(render_home(&posts)))
}

async fn post(
    State(state): State<AppState>,
    AxumPath(path): AxumPath<String>,
) -> Result<Html<String>, StatusCode> {
    let post = state
        .posts_repository
        .find_post(&path)
        .map_err(internal_server_error)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Html(render_post(&post)))
}

async fn about() -> Html<String> {
    Html(render_about())
}
