use std::path::Path;
use std::{fs, path::PathBuf, process::Command};

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::components::popup::ActivePopup;
use crate::utils::string::sanitize_filename;
use crate::{
    actions::navigation::{ListsView, View},
    app::App,
};

pub enum DownloadEvent {
    _Progress {
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

fn get_cookies_path(list_name: &str, base_dir: &Path) -> Option<PathBuf> {
    let config_path = base_dir
        .join("lists-configs")
        .join(list_name)
        .join("cookies.txt");

    if config_path.exists() {
        Some(config_path)
    } else {
        None
    }
}

async fn download_file_async(
    url: String,
    destination: PathBuf,
    tx: UnboundedSender<DownloadEvent>,
    cookies_path: Option<PathBuf>,
) {
    let mut cmd = tokio::process::Command::new("curl");

    cmd.arg("-L").arg("-o").arg(&destination).arg(&url);

    if let Some(cookie_file) = cookies_path {
        cmd.arg("-b").arg(cookie_file);
    }

    let output = cmd.output().await;

    match output {
        Ok(o) if o.status.success() => {
            let _ = tx.send(DownloadEvent::Finished);
        }
        _ => {
            let _ = tx.send(DownloadEvent::Error);
        }
    }
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
        let list_name = list.to_string();
        let base_dir = self.config.base_dir.clone();

        if item_path_clone.exists() {
            return self.execute_command();
        }

        let cookies_path = get_cookies_path(&list_name, &base_dir);

        tokio::spawn(
            async move { download_file_async(url, item_path_clone, tx, cookies_path).await },
        );
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

        let app_path = format!("{}", &base_dir.to_string_lossy());

        let download_dir = base_dir.join("downloads").join(list);
        let clean_title = sanitize_filename(&selected_item.item);
        let item_path = if self.navigation.list_view == ListsView::LocalLists {
            PathBuf::from(&selected_item.url)
        } else {
            download_dir.join(&clean_title)
        };

        let item_path = item_path.to_string_lossy();

        let mut command_str = selected_command.command.clone();
        command_str = command_str.replace("$EMUX", &app_path);
        command_str = command_str.replace("$ITEM", &item_path);
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
