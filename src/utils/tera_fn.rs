use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::SystemTime;
use tera::Value;
use tera::{to_value, try_get_value, Result};

pub fn date_format(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("date_format", "value", SystemTime, value);

    // 将 SystemTime 转换为 UTC DateTime
    let datetime: DateTime<Utc> = s.into();

    // 设置格式化字符串
    let format_string = "%Y-%m-%d %H:%M:%S";

    // 格式化日期时间
    let formatted_time = datetime.format(format_string).to_string();

    Ok(to_value(formatted_time)?)
}
