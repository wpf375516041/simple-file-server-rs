use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::SystemTime;
use tera::Value;
use tera::{to_value, try_get_value, Result};

pub fn date_format(value: &Value, args: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("date_format", "value", SystemTime, value);

    // 将 SystemTime 转换为 UTC DateTime
    let datetime: DateTime<Utc> = s.into();

    // 设置格式化字符串
    let format_string = match args.get("format") {
        Some(format) => try_get_value!("date_format", "format", String, format),
        None => String::from("%Y-%m-%d %H:%M:%S"),
    };

    // 格式化日期时间
    let formatted_time = datetime.format(format_string.as_str()).to_string();

    Ok(to_value(formatted_time)?)
}

// 自定义单位常量
const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

pub fn file_size_format(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let size: u64 = try_get_value!("file_size_format", "value", u64, value);

    // 计算指数并确保在有效范围内
    let index = (size as f64).log10() / 3.0;
    let index = index as usize;
    let index = if index > 4 { 4 } else { index };

    // 计算单位换算后的值
    let formatted_size = format!("{:.2}", size as f64 / 1024.0_f64.powi(index as i32));

    Ok(to_value(format!(
        "{} {}",
        formatted_size.trim_end_matches('0').trim_end_matches('.'),
        UNITS[index]
    ))?)
}
