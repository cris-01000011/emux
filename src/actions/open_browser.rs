use std::process::Stdio;

use crate::{actions::navigation::View, app::App, utils::string::clean_all};

impl App {
    pub fn open_browser_search(&self) {
        if self.navigation.view == View::Lists || self.get_current_list_items_count() == 0 {
            return;
        }

        let current_items = self.get_current_list_items();
        let selected_rom = &current_items[self.scroll.index_in_list()];

        let list = self.current_list_name();
        let clean_title = clean_all(&selected_rom.item);
        let clean_list = clean_all(list);

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
