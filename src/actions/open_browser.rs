use std::process::Stdio;

use crate::{actions::navigation::View, app::App};

impl App {
    fn normalize_game_title(title: &str) -> String {
        let without_ext = title
            .rsplit_once('.')
            .map(|(name, _)| name)
            .unwrap_or(title);

        let mut result = String::new();
        let mut depth = 0;

        for ch in without_ext.chars() {
            match ch {
                '(' => depth += 1,
                ')' if depth > 0 => depth -= 1,
                _ if depth == 0 => result.push(ch),
                _ => {}
            }
        }

        result.trim().to_string()
    }

    pub fn open_browser_search(&self) {
        if self.navigation.view == View::Lists || self.data.items_in_list.is_empty() {
            return;
        }

        let selected_rom =
            &self.data.items_in_list[self.ui_state.items_in_list.selected().unwrap_or(0)];

        let clean_title = Self::normalize_game_title(&selected_rom.item);
        let clean_list = Self::normalize_game_title(&self.navigation.current_list);

        let search_query = format!("{} {}", clean_title, clean_list);
        let encoded_query = urlencoding::encode(&search_query);

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;

            let _ = Command::new("xdg-open")
                .arg(format!(
                    "https://www.google.com/search?tbm=isch&q={}",
                    encoded_query
                ))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
        }
    }
}
