use chrono::{offset::Utc, DateTime};
use log::kv::{Key, VisitSource};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingInfo {
    pub dt: String,
    pub level: String,
    pub message: String,
    pub params: LoggingVisitor,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoggingVisitor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

impl<'kvs> VisitSource<'kvs> for LoggingVisitor {
    fn visit_pair(
        &mut self,
        key: Key<'kvs>,
        value: log::kv::Value<'kvs>,
    ) -> Result<(), log::kv::Error> {
        match key.as_str() {
            "app" => {
                self.app = Some(value.to_string());
            }
            "user" => {
                self.user = Some(value.to_string());
            }
            "status" => {
                self.status = Some(value.to_u64().unwrap() as u16);
            }
            "method" => {
                self.method = Some(value.to_string());
            }
            _ => {}
        };
        Ok(())
    }
}

pub fn init_logger() {
    env_logger::builder()
        .format(move |buf, record| {
            let datetime: DateTime<Utc> = SystemTime::now().into();
            let file_line = match (record.file(), record.line()) {
                (Some(file), Some(line)) if record.level() == log::Level::Error => {
                    format!(" -{file} {line}-")
                }
                _ => String::new(),
            };

            let mut params = LoggingVisitor::default();
            let _ = record.key_values().visit(&mut params);

            let message = format!(
                "{} {} {}",
                params.clone().method.unwrap_or_default(),
                params.status.unwrap_or_default(),
                record.args()
            );
            writeln!(
                buf,
                "{} [{}] {file_line} {message}",
                datetime.format("%T %D"),
                record.level()
            )
        })
        .init();
}

pub fn get_log_format() -> &'static str {
    "%s %r - %{r}a %Dms"
}
