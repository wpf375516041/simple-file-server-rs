use crate::utils::tera_fn::{date_format, file_size_format};
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use std::str::from_utf8;
use tera::Tera;

pub mod api;
pub mod utils;

#[derive(RustEmbed)]
#[folder = "templates/"] // 指定模板文件夹路径
struct TemplateFiles;

// 创建全局的 Tera 模板引擎 加载模板文件
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
         // 添加所有嵌入的模板到 Tera 引擎
        for path in TemplateFiles::iter() {
            let content = TemplateFiles::get(path.as_ref()).unwrap();
            tera.add_raw_template(path.as_ref(), from_utf8(content.as_ref()).unwrap()).unwrap();
        }
        tera.register_filter("date_format", date_format);
        tera.register_filter("file_size_format", file_size_format);
        tera
    };
}
