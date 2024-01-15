use crate::utils::fs_utils::get_directory_listing;
use crate::TEMPLATES;
use axum::extract::Query;
use axum::response::Html;
use std::collections::HashMap;
use std::path::Path as FS_Path;
use tera::Context;

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
