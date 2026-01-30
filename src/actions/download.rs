use std::{fs, path::PathBuf, process::Command};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::{io::AsyncWriteExt, sync::mpsc};
use tokio_stream::StreamExt;

use crate::components::popup::ActivePopup;
use crate::{actions::navigation::View, app::App};

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
    const PROGRESS_UPDATE_INTERVAL: u64 = 1024 * 1024; // Update every 1MB

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

        // Only send progress updates at intervals to avoid overwhelming the channel
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

    // Send final progress update to ensure 100% is displayed
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
    pub fn download_rom(&mut self) {
        if self.navigation.view == View::Lists || self.data.items_in_list.is_empty() {
            return;
        }

        self.download.progress = 0.0;
        self.download.downloaded = 0;
        self.download.total = 0;

        let (tx, rx) = mpsc::unbounded_channel();

        self.download.rx = Some(rx);

        let selected_rom = &self.data.items_in_list[self.ui.items_in_list.selected().unwrap_or(0)];

        let list = self.current_list_name();
        let download_dir = self.config.base_dir.join("downloads").join(list);
        let _ = fs::create_dir_all(&download_dir);

        let clean_title = Self::sanitize_filename(&selected_rom.item);
        let rom_path = download_dir.join(&clean_title);

        // Clone only what the async task needs
        let url = selected_rom.url.clone();
        let rom_path_clone = rom_path.clone();

        if rom_path_clone.exists() {
            return self.execute_command();
        }

        self.ui.popup.open(ActivePopup::Downloading);

        tokio::spawn(async move { download_file_async(url, rom_path_clone, tx).await });
    }

    pub fn execute_command(&mut self) {
        if self.navigation.view == View::Lists || self.data.items_in_list.is_empty() {
            return;
        }

        let commands = self.commands.get_current_commands();

        if commands.is_empty() {
            return;
        }

        if matches!(self.ui.popup.active, ActivePopup::Downloading)
            && self.download.progress < 100.0
        {
            return;
        }

        let selected_rom = &self.data.items_in_list[self.ui.items_in_list.selected().unwrap_or(0)];

        let selected_command = &commands[self.commands.selected_command];

        let base_dir = &self.config.base_dir;

        let list = self.current_list_name();
        let download_dir = base_dir.join("downloads").join(list);

        let clean_title = Self::sanitize_filename(&selected_rom.item);
        let rom_path = download_dir.join(&clean_title);
        let game_downloaded = rom_path.to_string_lossy();

        let retroarch_path = format!(
            "'{}/programs/retroarch/RetroArch-Linux-x86_64.AppImage'",
            &base_dir.to_string_lossy()
        );

        let duckstation_path = format!(
            "'{}/programs/duckstation/DuckStation-x64.AppImage'",
            &base_dir.to_string_lossy()
        );

        let ppsspp_app = format!(
            "'{}/programs/ppsspp/PPSSPP-v1.19.3-anylinux-x86_64.AppImage'",
            &base_dir.to_string_lossy()
        );

        let ppsspp_path = format!("'{}/programs/ppsspp'", &base_dir.to_string_lossy());

        let mut command_str = selected_command.command.clone();
        command_str = command_str.replace("$RETROARCH", &retroarch_path);
        command_str = command_str.replace("$DUCKSTATION", &duckstation_path);
        command_str = command_str.replace("$PPSSPP_PATH", &ppsspp_path);
        command_str = command_str.replace("$PPSSPP", &ppsspp_app);
        command_str = command_str.replace("$GAME_DOWNLOADED", &game_downloaded);
        command_str.push_str(" >/dev/null 2>&1 &");

        #[cfg(unix)]
        {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(&command_str);
            let _ = cmd.spawn();
        }

        self.ui.popup.close();
    }

    fn sanitize_filename(filename: &str) -> String {
        filename
            .replace('\'', "")
            .replace('\"', "")
            .replace('/', "_")
            .replace('\\', "_")
            .replace(':', "_")
            .replace('*', "_")
            .replace('?', "_")
            .replace('<', "_")
            .replace('>', "_")
            .replace('|', "_")
    }
}
