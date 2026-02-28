use std::path::PathBuf;

use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use url::Url;

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
    input_url: String,
    tx: UnboundedSender<CreateListEvent>,
    lists_dir: String,
) {
    let client = Client::new();
    let html = match client.get(&input_url).send().await {
        Ok(r) => r,
        Err(_) => {
            let _ = tx.send(CreateListEvent::Error);
            return;
        }
    };

    let html = match html.text().await {
        Ok(h) => h,
        Err(_) => {
            let _ = tx.send(CreateListEvent::Error);
            return;
        }
    };

    let document = Html::parse_document(&html);
    let selector = Selector::parse("a").unwrap();

    let base = match Url::parse(&input_url) {
        Ok(b) => b,
        Err(_) => {
            let _ = tx.send(CreateListEvent::Error);
            return;
        }
    };

    let mut items = Vec::new();
    let elements: Vec<_> = document.select(&selector).collect();
    let total = elements.len();

    let include: Vec<&str> = vec![".zip", ".chd", ".iso", ".7z", ".rar"];

    for (current, element) in elements.into_iter().enumerate() {
        let text = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();

        if text.is_empty() {
            continue;
        }

        if let Some(href) = element.value().attr("href")
            && let Ok(full_url) = base.join(href)
        {
            let url_str = full_url.as_str().to_lowercase();

            if include.iter().any(|ext| url_str.ends_with(ext)) {
                items.push(LinkItem {
                    item: text,
                    url: full_url.to_string(),
                });
            }
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
        let input_url = self.ui.input.new_list_url.value();

        if input_name.is_empty() || input_url.is_empty() {
            return;
        }

        self.create_list.current = 0;
        self.create_list.total = 0;
        self.create_list.found = 0;

        let (tx, rx) = mpsc::unbounded_channel();

        self.create_list.rx = Some(rx);

        let lists_dir = self.config.lists_dir.to_string_lossy().to_string();
        let input_name = input_name.to_string();
        let input_url = input_url.to_string();

        tokio::spawn(async move {
            create_list_async(input_name, input_url, tx, lists_dir).await;
        });
    }
}
