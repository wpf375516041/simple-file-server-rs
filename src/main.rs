use axum::routing::get;
use axum::Router;
use rust_embed::RustEmbed;
use simple_file_server_rs::api::fs_api::home_dir;
use tower_http::services::ServeDir;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[derive(RustEmbed)]
#[folder = "templates/"] // 指定模板文件夹路径
struct TemplateFiles;

#[tokio::main]
async fn main() {
    // http日志打印美化
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home_dir))
        // 让css js能够渲染
        .nest_service("/templates", ServeDir::new("templates"))
        .layer(
            // 添加http日志打印中间件
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );
    // run our app with hyper, listening globally on port 8080
    info!("Run app Listening on port 8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
