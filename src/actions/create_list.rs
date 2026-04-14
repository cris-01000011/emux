use std::path::PathBuf;

use serde::Serialize;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use urlencoding;

use crate::app::App;

#[derive(Serialize)]
struct LinkItem {
    item: String,
    url: String,
}

pub enum CreateListEvent {
    Progress {
        current: usize,
        total: usize,
        found: usize,
    },
    Finished,
    Error,
}

#[derive(Default)]
pub struct CreateList {
    pub rx: Option<UnboundedReceiver<CreateListEvent>>,
    pub current: usize,
    pub total: usize,
    pub found: usize,
}

async fn create_list_async(
    input_name: String,
    curl_args: String,
    tx: UnboundedSender<CreateListEvent>,
    lists_dir: String,
) {
    let output = Command::new("sh")
        .args(["-c", &format!("curl {}", curl_args)])
        .output()
        .await;

    let output = match output {
        Ok(o) => o,
        Err(_) => {
            let _ = tx.send(CreateListEvent::Error);
            return;
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let include: Vec<&str> = vec![".zip", ".chd", ".iso", ".7z", ".rar"];

    let lines: Vec<&str> = stdout.lines().collect();
    let total = lines.len();

    let mut items = Vec::new();

    for (current, line) in lines.iter().enumerate() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let url_lower = line.to_lowercase();

        if include.iter().any(|ext| url_lower.ends_with(ext)) {
            let decode = urlencoding::decode(line)
                .unwrap_or_else(|_| line.into())
                .into_owned();

            let item = decode.rsplit('/').next().unwrap_or(&decode).to_string();

            items.push(LinkItem {
                item,
                url: line.to_string(),
            });
        }

        let _ = tx.send(CreateListEvent::Progress {
            current,
            total,
            found: items.len(),
        });
    }

    let json = match serde_json::to_string_pretty(&items) {
        Ok(j) => j,
        Err(_) => {
            let _ = tx.send(CreateListEvent::Error);
            return;
        }
    };

    let mut path = PathBuf::from(&lists_dir);
    path.push(format!("{input_name}.json"));

    if std::fs::write(path, json).is_err() {
        let _ = tx.send(CreateListEvent::Error);
        return;
    }

    let _ = tx.send(CreateListEvent::Finished);
}

impl App {
    pub fn create_list(&mut self) {
        let input_name = self.ui.input.new_list_name.value();
        let curl_args = self.ui.input.new_list_url.value();

        if input_name.is_empty() || curl_args.is_empty() {
            return;
        }

        self.create_list.current = 0;
        self.create_list.total = 0;
        self.create_list.found = 0;

        let (tx, rx) = mpsc::unbounded_channel();

        self.create_list.rx = Some(rx);

        let lists_dir = self.config.lists_dir.to_string_lossy().to_string();
        let input_name = input_name.to_string();
        let curl_args = curl_args.to_string();

        tokio::spawn(async move {
            create_list_async(input_name, curl_args, tx, lists_dir).await;
        });
    }
}
