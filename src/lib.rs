use lazy_static::lazy_static;
use tera::Tera;

pub mod api;
pub mod utils;

// 创建全局的 Tera 模板引擎 加载模板文件
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        tera
    };
}
