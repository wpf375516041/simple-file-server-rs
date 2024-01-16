use crate::utils::fs_utils::get_directory_listing;
use crate::TEMPLATES;
use axum::body::Body;
use axum::extract::{Path, Query, Request};
use axum::response::{Html, IntoResponse};
use http::{header, HeaderMap, StatusCode};
use std::collections::HashMap;
use std::path::{Path as FS_Path, PathBuf};
use tera::Context;
use tower_http::services::ServeFile;

// 文件列表接口
pub async fn home_dir(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let dir = match params.get("dir") {
        Some(dir) => dir,
        None => "/",
    };
    // 创建上下文并填充数据
    let mut context = Context::new();
    context.insert("dirs", &get_directory_listing(FS_Path::new(&dir)));
    context.insert("title", "文件服务器");
    context.insert("parent_dir", &dir);
    // 渲染模板
    Html(TEMPLATES.render("index.html", &context).unwrap())
}

// 实现文件下载路由
pub async fn file_download(Path(file): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(file);

    let filename = match path.file_name() {
        Some(name) => name,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "File name couldn't be determined".to_string(),
            ))
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{:?}\"", filename)
            .parse()
            .unwrap(),
    );
    let mut req = Request::new(Body::empty());
    *req.headers_mut() = headers;
    Ok(ServeFile::new(path).try_call(req).await.unwrap())
}
