use std::{fs, path::PathBuf, process::Command};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::{io::AsyncWriteExt, sync::mpsc};
use tokio_stream::StreamExt;

use crate::components::popup::ActivePopup;
use crate::utils::string::sanitize_filename;
use crate::{
    actions::navigation::{ListsView, View},
    app::App,
};

pub enum DownloadEvent {
    Progress {
        percent: f64,
        downloaded: u64,
        total: u64,
    },
    Finished,
    Error,
}

#[derive(Default)]
pub struct Download {
    pub rx: Option<UnboundedReceiver<DownloadEvent>>,
    pub progress: f64,
    pub downloaded: u64,
    pub total: u64,
}

async fn download_file_async(
    url: String,
    destination: PathBuf,
    tx: UnboundedSender<DownloadEvent>,
) {
    let client = reqwest::Client::new();

    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(_) => {
            let _ = tx.send(DownloadEvent::Error);
            return;
        }
    };

    let total_size = response.content_length();
    let mut downloaded: u64 = 0;
    let mut last_progress_update: u64 = 0;
    const PROGRESS_UPDATE_INTERVAL: u64 = 1024 * 1024;

    let mut file = match tokio::fs::File::create(&destination).await {
        Ok(f) => f,
        Err(_) => {
            let _ = tx.send(DownloadEvent::Error);
            return;
        }
    };

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = match chunk {
            Ok(c) => c,
            Err(_) => {
                let _ = tx.send(DownloadEvent::Error);
                return;
            }
        };

        if file.write_all(&chunk).await.is_err() {
            let _ = tx.send(DownloadEvent::Error);
            return;
        }

        downloaded += chunk.len() as u64;

        if downloaded >= last_progress_update + PROGRESS_UPDATE_INTERVAL {
            let percent = match total_size {
                Some(total) if total > 0 => (downloaded as f64 / total as f64).clamp(0.0, 1.0),
                _ => 0.0,
            };

            let _ = tx.send(DownloadEvent::Progress {
                percent,
                downloaded,
                total: total_size.unwrap_or(0),
            });
            last_progress_update = downloaded;
        }
    }

    let percent = match total_size {
        Some(total) if total > 0 => (downloaded as f64 / total as f64).clamp(0.0, 1.0),
        _ => 1.0,
    };

    let _ = tx.send(DownloadEvent::Progress {
        percent,
        downloaded,
        total: total_size.unwrap_or(0),
    });

    let _ = tx.send(DownloadEvent::Finished);
}

impl App {
    pub fn download_item(&mut self) {
        if self.navigation.view == View::Lists || self.get_current_list_items_count() == 0 {
            return;
        }

        if self.navigation.list_view == ListsView::LocalLists {
            return self.execute_command();
        }

        self.download.progress = 0.0;
        self.download.downloaded = 0;
        self.download.total = 0;

        let (tx, rx) = mpsc::unbounded_channel();

        self.download.rx = Some(rx);

        let current_items = self.get_current_list_items();
        let selected_item = &current_items[self.scroll.index_in_list()];

        let list = self.current_list_name();
        let download_dir = self.config.base_dir.join("downloads").join(list);
        let _ = fs::create_dir_all(&download_dir);

        let clean_title = sanitize_filename(&selected_item.item);
        let item_path = download_dir.join(&clean_title);

        let url = selected_item.url.clone();
        let item_path_clone = item_path.clone();

        if item_path_clone.exists() {
            return self.execute_command();
        }

        self.ui.popup.open(ActivePopup::Downloading);

        tokio::spawn(async move { download_file_async(url, item_path_clone, tx).await });
    }

    pub fn execute_command(&mut self) {
        if self.navigation.view == View::Lists || self.get_current_list_items_count() == 0 {
            return;
        }

        let command_count = self.commands.get_current_list_len();

        if command_count == 0 {
            return;
        }

        if matches!(self.ui.popup.active, ActivePopup::Downloading)
            && self.download.progress < 100.0
        {
            return;
        }

        let current_items = self.get_current_list_items();
        let selected_item = &current_items[self.scroll.index_in_list()];

        let selected_command = &self.commands.lists[self.commands.selected_list].commands
            [self.commands.selected_command];

        let base_dir = &self.config.base_dir;

        let list = self.current_list_name();

        let scripts_path = format!("{}/scripts", &base_dir.to_string_lossy());

        let download_dir = base_dir.join("downloads").join(list);
        let clean_title = sanitize_filename(&selected_item.item);
        let item_path = if self.navigation.list_view == ListsView::LocalLists {
            PathBuf::from(&selected_item.url)
        } else {
            download_dir.join(&clean_title)
        };

        let item_downloaded = item_path.to_string_lossy();

        let mut command_str = selected_command.command.clone();
        command_str = command_str.replace("$SCRIPTS", &scripts_path);
        command_str = command_str.replace("$ITEM_DOWNLOADED", &item_downloaded);
        command_str.push_str(" >/dev/null 2>&1 &");

        #[cfg(unix)]
        {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(&command_str);
            let _ = cmd.spawn();
        }

        self.ui.popup.close();
    }
}
