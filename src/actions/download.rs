use std::{fs, process::Command};

use crate::app::App;

impl App {
    pub fn download_rom(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.in_list || self.items_in_list.is_empty() {
            return Ok(());
        }

        let selected_rom = &self.items_in_list[self.items_in_list_state.selected().unwrap_or(0)];

        let download_dir = Self::emux_base_path()
            .join("downloads")
            .join(&self.current_list);
        fs::create_dir_all(&download_dir)?;

        let clean_title = Self::sanitize_filename(&selected_rom.item);
        let rom_path = download_dir.join(&clean_title);

        if rom_path.exists() {
            return self.execute_command();
        }

        let client = reqwest::blocking::Client::builder()
            .no_gzip()
            .no_brotli()
            .build()?;

        let mut response = client.get(&selected_rom.url).send()?;
        let mut file = fs::File::create(&rom_path)?;

        std::io::copy(&mut response, &mut file)?;

        self.execute_command()
    }

    pub fn execute_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.in_list || self.items_in_list.is_empty() {
            return Ok(());
        }

        let selected_rom = &self.items_in_list[self.items_in_list_state.selected().unwrap_or(0)];
        let commands = self.get_current_commands();

        if commands.is_empty() {
            return Ok(());
        }

        let selected_command = &commands[self.selected_command];

        // Get paths for variable substitution
        let emux_path = Self::emux_base_path();
        let download_dir = emux_path.join("downloads").join(&self.current_list);
        let clean_title = Self::sanitize_filename(&selected_rom.item);
        let rom_path = download_dir.join(&clean_title);
        let game_downloaded = rom_path.to_string_lossy();

        let retroarch_path = format!(
            "'{}/programs/retroarch/RetroArch-Linux-x86_64.AppImage'",
            &emux_path.to_string_lossy()
        );

        let duckstation_path = format!(
            "'{}/programs/duckstation/DuckStation-x64.AppImage'",
            &emux_path.to_string_lossy()
        );

        let ppsspp_app = format!(
            "'{}/programs/ppsspp/PPSSPP-v1.19.3-anylinux-x86_64.AppImage'",
            &emux_path.to_string_lossy()
        );

        let ppsspp_path = format!("'{}/programs/ppsspp'", &emux_path.to_string_lossy());

        // Prepare command with variable substitution
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
            cmd.spawn()?;
        }

        Ok(())
    }

    fn sanitize_filename(filename: &str) -> String {
        // Replace potentially problematic characters for file paths
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
