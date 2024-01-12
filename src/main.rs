use axum::response::Html;
use axum::routing::get;
use axum::Router;
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use tera::{Context, Tera};
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[derive(RustEmbed)]
#[folder = "templates/"] // 指定模板文件夹路径
struct TemplateFiles;

// 创建全局的 Tera 模板引擎 加载模板文件
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        tera
    };
}

#[tokio::main]
async fn main() {
    // http日志打印美化
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // build our application with a single route
    let app = Router::new().route("/", get(hello_world)).layer(
        // 添加http日志打印中间件
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<String> {
    // 创建上下文并填充数据
    let mut context = Context::new();
    context.insert("title", "Hello World");
    context.insert("content", "This is a dynamically generated page.");
    // 渲染模板
    Html(TEMPLATES.render("index.html", &context).unwrap())
}
