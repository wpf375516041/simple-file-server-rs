use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use simple_file_server_rs::api::fs_api::{file_download, file_upload, home_dir};
use std::net::IpAddr;
use structopt::StructOpt;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[derive(Debug, StructOpt)]
struct CliOptions {
    /// Server port
    #[structopt(short, long, default_value = "80")]
    port: u16,
}

#[axum_static_include::static_serve("templates")]
fn assert_fold() -> axum::Router {}

#[tokio::main]
async fn main() {
    // 解析命令行参数
    let cli_options = CliOptions::from_args();
    let local_ip: IpAddr = get_local_ip().unwrap();
    let asserts = assert_fold();

    // http日志打印美化
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home_dir))
        .route("/download/:file", get(file_download))
        .route("/upload", post(file_upload))
        // 让css js能够渲染
        .nest_service("/templates", asserts)
        .layer(
            // 添加http日志打印中间件
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024));
    // run our app with hyper, listening globally on port 8080
    info!("Run app Listening on {}:{}", local_ip, cli_options.port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cli_options.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_local_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
    // 使用网络库来获取本机的IP地址
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_ip = socket.local_addr()?.ip();

    Ok(local_ip)
}
