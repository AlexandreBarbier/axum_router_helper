use chrono::{offset::Utc, DateTime};
use log::kv::{Key, VisitSource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<HashMap<String, String>>,
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
                self.status = Some(value.to_u64().unwrap_or_default() as u16);
            }
            "method" => {
                self.method = Some(value.to_string());
            }

            _ => {
                if self.context.is_none() {
                    self.context = Some(HashMap::new());
                }
                self.context
                    .as_mut()
                    .unwrap()
                    .insert(key.as_str().to_string(), value.to_string());
            }
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

pub fn init_remote_logger(with_better_stack: bool, global_params: Option<HashMap<String, String>>) {
    std::env::var("BETTERSTACK_API_KEY").expect("BETTERSTACK_API_KEY is needed");
    let (tx, rx) = std::sync::mpsc::channel::<LoggingInfo>();
    if with_better_stack && std::env::var("BETTERSTACK_API_KEY").is_ok() {
        tokio::spawn(async move {
            for receive in rx {
                send_to_better_stack(receive);
            }
        });
    }
    env_logger::builder()
        .format(move |buf, record| {
            let datetime: DateTime<Utc> = SystemTime::now().into();
            let file_line = match (record.file(), record.line()) {
                (Some(file), Some(line)) if record.level() == log::Level::Error => {
                    format!(" -{file} {line}-")
                }
                _ => String::new(),
            };
            let level = record.level().to_string();
            let args = format!("{}", record.args());
            let mut params = LoggingVisitor::default();
            if let Some(global_params) = &global_params {
                for (key, value) in global_params {
                    params
                        .context
                        .get_or_insert_with(HashMap::new)
                        .insert(key.clone(), value.clone());
                }
            }
            let _ = record.key_values().visit(&mut params);
            if with_better_stack && std::env::var("BETTERSTACK_API_KEY").is_ok() {
                let _ = tx.send(LoggingInfo {
                    level,
                    dt: datetime.to_string(),
                    message: args.clone(),
                    params: params.clone(),
                });
            }
            let mut message = "".to_string();
            if let Some(method) = params.clone().method {
                message += method.as_str();
                message += " ";
            }

            if params.status.unwrap_or_default() != 0 {
                let status = params.status.unwrap();
                message += status.to_string().as_str();
                message += " ";
            }

            message += &args;
            writeln!(
                buf,
                "{} [{}] {} {}",
                datetime.format("%T %D"),
                record.level(),
                file_line,
                message
            )
        })
        .init();
}

pub fn send_to_better_stack(message: LoggingInfo) {
    let buf = rmp_serde::encode::to_vec(&serde_json::json!(message)).unwrap();
    let url = std::env::var("BETTERSTACK_URL").expect("BETTERSTACK_URL is needed");
    let _ = ureq::post(url.as_str())
        .header("Content-Type", "application/msgpack")
        .header("Accept", "application/json, text/plain")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                std::env::var("BETTERSTACK_API_KEY").expect("BETTERSTACK_API_KEY must be set")
            )
            .as_str(),
        )
        .send(&buf);
}

pub fn get_log_format() -> &'static str {
    "%s %r - %{r}a %Dms"
}
