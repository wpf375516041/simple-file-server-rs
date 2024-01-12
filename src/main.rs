use axum::response::{Html};
use axum::Router;
use axum::routing::get;
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use tera::{Context, Tera};

#[derive(RustEmbed)]
#[folder = "templates/"] // 指定模板文件夹路径
struct TemplateFiles;


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        tera
    };
}


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(hello_world));

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

