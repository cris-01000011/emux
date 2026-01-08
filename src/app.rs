use home;
use rand::Rng;
use serde::Deserialize;
use std::{
    fs,
    path::PathBuf,
    process::{Command as ProcessCommand, Stdio},
};

#[derive(Deserialize, Debug)]
struct EmuxConfig {
    paths: PathsConfig,
}

#[derive(Deserialize, Debug)]
struct PathsConfig {
    root: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RomEntry {
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub command: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SystemCommand {
    pub system: String,
    pub commands: Vec<Command>,
}

#[derive(Deserialize, Debug, Clone, serde::Serialize)]
pub struct FavoriteEntry {
    pub system: String,
    pub title: String,
}

pub struct App {
    pub current_path: PathBuf,
    pub entries: Vec<PathBuf>,
    pub selected: usize,
    pub scroll_offset: usize,

    pub in_system: bool,
    pub roms: Vec<RomEntry>,
    pub current_system: String,
    pub system_commands: Vec<SystemCommand>,
    pub selected_command: usize,
    pub in_command_selection: bool,
    pub roms_scroll_offset: usize,

    // Search state
    pub in_search_mode: bool,
    pub search_query: String,
    pub search_results: Vec<usize>, // Original indices of matching items
    pub search_selected: usize,     // Selected index in search results

    // Favorites state
    pub favorites: Vec<FavoriteEntry>,

    // Memory for last selections
    pub directory_selections: std::collections::HashMap<String, usize>,
    pub system_selections: std::collections::HashMap<String, usize>,
    pub command_selections: std::collections::HashMap<String, usize>,
    pub directory_scroll_selections: std::collections::HashMap<String, usize>,
    pub system_scroll_selections: std::collections::HashMap<String, usize>,
}

impl App {
    fn emux_base_path() -> PathBuf {
        // Try to read config from ~/.config/emux/emux.toml
        if let Some(home) = home::home_dir() {
            let config_path = home.join(".config/emux/emux.toml");
            if let Ok(config_content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = toml::from_str::<EmuxConfig>(&config_content) {
                    if config.paths.root != "default" {
                        return PathBuf::from(config.paths.root);
                    }
                }
            }
            // Fallback to default
            home.join("Emux")
        } else {
            PathBuf::from("./Emux")
        }
    }

    fn games_path() -> PathBuf {
        Self::emux_base_path().join("games")
    }

    pub fn new() -> Self {
        let start = Self::games_path();

        let mut app = App {
            current_path: start.clone(),
            entries: Vec::new(),
            selected: 0,
            scroll_offset: 0,

            in_system: false,
            roms: Vec::new(),
            current_system: String::new(),
            system_commands: Vec::new(),
            selected_command: 0,
            in_command_selection: false,
            roms_scroll_offset: 0,

            in_search_mode: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,

            favorites: Vec::new(),

            directory_selections: std::collections::HashMap::new(),
            system_selections: std::collections::HashMap::new(),
            command_selections: std::collections::HashMap::new(),
            directory_scroll_selections: std::collections::HashMap::new(),
            system_scroll_selections: std::collections::HashMap::new(),
        };

        app.load_dir(start);
        app.load_system_commands();
        app.load_favorites();

        app
    }

    pub fn load_dir(&mut self, dir: PathBuf) {
        // Save current selection and scroll before loading new directory
        if !self.in_system {
            let path_str = self.current_path.to_string_lossy().to_string();
            self.directory_selections
                .insert(path_str.clone(), self.selected);
            self.directory_scroll_selections
                .insert(path_str, self.scroll_offset);
        }

        self.entries = fs::read_dir(&dir)
            .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).collect())
            .unwrap_or_default();

        self.entries.sort();

        self.current_path = dir.clone();

        // Restore saved selection for this directory, default to 0 if not found
        let path_str = self.current_path.to_string_lossy().to_string();
        self.selected = self
            .directory_selections
            .get(&path_str)
            .copied()
            .unwrap_or(0);

        // Restore saved scroll offset for this directory, default to 0 if not found
        self.scroll_offset = self
            .directory_scroll_selections
            .get(&path_str)
            .copied()
            .unwrap_or(0);

        self.in_system = false;
        self.roms.clear();
        self.current_system.clear();
        self.selected_command = 0;
        self.in_command_selection = false;
        self.scroll_offset = 0;
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = if self.in_system {
                self.roms.len().saturating_sub(1)
            } else {
                self.entries.len().saturating_sub(1)
            };
        }
        // Will be updated from UI with actual visible height
    }

    pub fn move_down(&mut self) {
        if self.in_system {
            if self.selected + 1 < self.roms.len() {
                self.selected += 1;
            } else {
                self.selected = 0;
            }
        } else if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        } else {
            self.selected = 0;
        }
        // Will be updated from UI with actual visible height
    }

    pub fn jump_to_random(&mut self) {
        if self.in_system {
            if !self.roms.is_empty() {
                self.selected = rand::thread_rng().gen_range(0..self.roms.len());
            }
        } else {
            if !self.entries.is_empty() {
                self.selected = rand::thread_rng().gen_range(0..self.entries.len());
            }
        }
    }

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
        if !self.in_system || self.roms.is_empty() {
            return;
        }

        let selected_rom = &self.roms[self.selected];

        let clean_title = Self::normalize_game_title(&selected_rom.title);
        let clean_system = Self::normalize_game_title(&self.current_system);

        let search_query = format!("{} {}", clean_title, clean_system);
        let encoded_query = urlencoding::encode(&search_query);

        #[cfg(target_os = "linux")]
        {
            let _ = ProcessCommand::new("xdg-open")
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

    fn update_scroll_offset(&mut self, max_visible_items: usize) {
        if self.in_system {
            if self.selected < self.roms_scroll_offset {
                self.roms_scroll_offset = self.selected;
            } else if self.selected >= self.roms_scroll_offset + max_visible_items.saturating_sub(2)
            {
                // Start scrolling 2 items before the end (ante-penultimate)
                self.roms_scroll_offset = self
                    .selected
                    .saturating_sub(max_visible_items.saturating_sub(2) - 1);
            }
        } else {
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            } else if self.selected >= self.scroll_offset + max_visible_items.saturating_sub(2) {
                // Start scrolling 2 items before the end (ante-penultimate)
                self.scroll_offset = self
                    .selected
                    .saturating_sub(max_visible_items.saturating_sub(2) - 1);
            }
        }
    }

    pub fn enter(&mut self) {
        if self.in_system {
            // Download ROM to current directory
            if let Err(e) = self.download_rom() {
                eprintln!("Error downloading ROM: {}", e);
            }
            return;
        }

        if let Some(path) = self.entries.get(self.selected).cloned() {
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                self.load_json_system(path);
                return;
            }
        }
    }

    fn load_json_system(&mut self, path: PathBuf) {
        // Save current directory selection and scroll before entering system
        let path_str = self.current_path.to_string_lossy().to_string();
        self.directory_selections
            .insert(path_str.clone(), self.selected);
        self.directory_scroll_selections
            .insert(path_str, self.scroll_offset);

        let data = std::fs::read_to_string(&path).unwrap_or_default();
        match serde_json::from_str::<Vec<RomEntry>>(&data) {
            Ok(roms) => {
                self.roms = roms;
            }
            Err(e) => {
                eprintln!("Error parsing JSON for {}: {}", path.display(), e);
                self.roms = Vec::new();
            }
        }

        // Extract system name from filename (handle quotes properly)
        self.current_system = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.in_system = true;

        // Restore saved selections for this system
        self.selected = self
            .system_selections
            .get(&self.current_system)
            .copied()
            .unwrap_or(0);
        self.selected_command = self
            .command_selections
            .get(&self.current_system)
            .copied()
            .unwrap_or(0);
        self.in_command_selection = true; // Auto-select first command

        // Restore saved ROM scroll offset for this system, default to 0 if not found
        self.roms_scroll_offset = self
            .system_scroll_selections
            .get(&self.current_system)
            .copied()
            .unwrap_or(0);
    }

    pub fn go_back(&mut self) {
        if self.in_system {
            // Save current selections and scroll before leaving system
            self.system_selections
                .insert(self.current_system.clone(), self.selected);
            self.command_selections
                .insert(self.current_system.clone(), self.selected_command);
            self.system_scroll_selections
                .insert(self.current_system.clone(), self.roms_scroll_offset);

            // return to system list
            self.in_system = false;
            self.roms.clear();
            self.current_system.clear();
            self.selected_command = 0;
            self.in_command_selection = false;
            self.roms_scroll_offset = 0; // Reset ROM scroll offset

            // Restore directory selection for this path
            let path_str = self.current_path.to_string_lossy().to_string();
            self.selected = self
                .directory_selections
                .get(&path_str)
                .copied()
                .unwrap_or(0);
            return;
        }

        if self.current_path == Self::games_path() {
            return;
        }

        if let Some(parent) = self.current_path.parent() {
            if parent.starts_with(Self::games_path()) {
                self.load_dir(parent.to_path_buf());
            }
        }
    }

    pub fn download_rom(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.in_system || self.roms.is_empty() {
            return Ok(());
        }

        let selected_rom = &self.roms[self.selected];

        // Create download directory if it doesn't exist
        let download_dir = Self::emux_base_path()
            .join("downloaded-games")
            .join(&self.current_system);
        fs::create_dir_all(&download_dir)?;

        let clean_title = Self::sanitize_filename(&selected_rom.title);
        let rom_path = download_dir.join(&clean_title);

        // Check if ROM already exists
        if rom_path.exists() {
            // Execute command if ROM already exists
            return self.execute_command();
        }

        let response = reqwest::blocking::get(&selected_rom.url)?;
        let mut file = fs::File::create(&rom_path)?;
        std::io::copy(&mut response.bytes()?.as_ref(), &mut file)?;

        // Execute command after download completes
        self.execute_command()
    }

    pub fn execute_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.in_system || self.roms.is_empty() {
            return Ok(());
        }

        let selected_rom = &self.roms[self.selected];
        let commands = self.get_current_commands();

        if commands.is_empty() {
            return Ok(());
        }

        let selected_command = &commands[self.selected_command];

        // Get paths for variable substitution
        let emux_path = Self::emux_base_path();
        let download_dir = emux_path
            .join("downloaded-games")
            .join(&self.current_system);
        let clean_title = Self::sanitize_filename(&selected_rom.title);
        let rom_path = download_dir.join(&clean_title);
        let game_downloaded = rom_path.to_string_lossy();

        let retroarch_path = format!(
            "'{}/emulators/retroarch/RetroArch-Linux-x86_64.AppImage'",
            &emux_path.to_string_lossy()
        );

        // Prepare command with variable substitution
        let mut command_str = selected_command.command.clone();
        command_str = command_str.replace("$RETROARCH", &retroarch_path);
        command_str = command_str.replace("$GAME_DOWNLOADED", &game_downloaded);
        command_str.push_str(" >/dev/null 2>&1 &");

        #[cfg(unix)]
        {
            let mut cmd = ProcessCommand::new("sh");
            cmd.arg("-c").arg(&command_str);
            cmd.spawn()?;
        }

        Ok(())
    }

    pub fn load_system_commands(&mut self) {
        let commands_path = Self::emux_base_path().join("system_commands.json");
        let data = std::fs::read_to_string(&commands_path).unwrap_or_default();
        self.system_commands = serde_json::from_str(&data).unwrap_or_default();
    }

    fn clean_system_name(system: &str) -> String {
        system
            .split('(')
            .next()
            .unwrap_or(system)
            .trim()
            .to_string()
    }

    fn sanitize_filename(filename: &str) -> String {
        // Replace potentially problematic characters for file paths
        filename
            .replace('\'', "") // Remove single quotes
            .replace('\"', "") // Remove double quotes
            .replace('/', "_") // Replace forward slashes
            .replace('\\', "_") // Replace backslashes
            .replace(':', "_") // Replace colons
            .replace('*', "_") // Replace asterisks
            .replace('?', "_") // Replace question marks
            .replace('<', "_") // Replace less than
            .replace('>', "_") // Replace greater than
            .replace('|', "_") // Replace pipes
    }

    pub fn get_current_commands(&self) -> Vec<Command> {
        if self.current_system.is_empty() {
            return Vec::new();
        }

        let clean_system = Self::clean_system_name(&self.current_system);

        self.system_commands
            .iter()
            .find(|sc| sc.system == clean_system)
            .map(|sc| sc.commands.clone())
            .unwrap_or_default()
    }

    pub fn toggle_command_selection(&mut self) {
        if self.in_system && !self.get_current_commands().is_empty() {
            self.in_command_selection = !self.in_command_selection;
        }
    }

    pub fn next_command(&mut self) {
        if !self.in_command_selection {
            return;
        }

        let commands = self.get_current_commands();
        if !commands.is_empty() {
            self.selected_command = (self.selected_command + 1) % commands.len();
        }
    }

    pub fn prev_command(&mut self) {
        if !self.in_command_selection {
            return;
        }

        let commands = self.get_current_commands();
        if !commands.is_empty() {
            self.selected_command = if self.selected_command == 0 {
                commands.len() - 1
            } else {
                self.selected_command - 1
            };
        }
    }

    pub fn go_to_first_item(&mut self) {
        self.selected = 0;
    }

    pub fn go_to_last_item(&mut self) {
        if self.in_system {
            if !self.roms.is_empty() {
                self.selected = self.roms.len().saturating_sub(1);
            }
        } else {
            if !self.entries.is_empty() {
                self.selected = self.entries.len().saturating_sub(1);
            }
        }
    }

    pub fn start_search(&mut self) {
        self.in_search_mode = true;
        self.search_query.clear();
        self.search_results.clear();
        self.search_selected = 0;
    }

    pub fn stop_search(&mut self) {
        if let Some(original_index) = self.get_current_search_index() {
            self.selected = original_index;
        }
        self.in_search_mode = false;
        self.search_query.clear();
        self.search_results.clear();
    }

    pub fn add_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.update_search_results();
    }

    pub fn remove_search_char(&mut self) {
        self.search_query.pop();
        self.update_search_results();
    }

    fn update_search_results(&mut self) {
        if self.search_query.is_empty() {
            // When search query is empty, show all items (no filtering)
            if self.in_system {
                // Show all ROMs
                self.search_results = (0..self.roms.len()).collect();
            } else {
                // Show all systems
                self.search_results = (0..self.entries.len()).collect();
            }
            return;
        }

        let query_lower = self.search_query.to_lowercase();

        if self.in_system {
            // Search in ROMs
            self.search_results = self
                .roms
                .iter()
                .enumerate()
                .filter(|(_, rom)| rom.title.to_lowercase().contains(&query_lower))
                .map(|(index, _)| index)
                .collect();
        } else {
            // Search in systems (entries)
            self.search_results = self
                .entries
                .iter()
                .enumerate()
                .filter(|(_, path)| {
                    path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query_lower)
                })
                .map(|(index, _)| index)
                .collect();
        }

        // Reset search selection
        self.search_selected = 0;
    }

    pub fn get_current_search_index(&self) -> Option<usize> {
        self.search_results.get(self.search_selected).copied()
    }

    pub fn search_up(&mut self) {
        if !self.search_results.is_empty() {
            self.search_selected = if self.search_selected == 0 {
                self.search_results.len() - 1
            } else {
                self.search_selected - 1
            };
        }
    }

    pub fn search_down(&mut self) {
        if !self.search_results.is_empty() {
            self.search_selected = (self.search_selected + 1) % self.search_results.len();
        }
    }

    fn favorites_path() -> PathBuf {
        Self::emux_base_path().join("lists").join("favorites.json")
    }

    pub fn load_favorites(&mut self) {
        let favorites_path = Self::favorites_path();
        if let Ok(data) = std::fs::read_to_string(&favorites_path) {
            if let Ok(favorites) = serde_json::from_str::<Vec<FavoriteEntry>>(&data) {
                self.favorites = favorites;
            }
        }
    }

    pub fn save_favorites(&self) {
        let favorites_path = Self::favorites_path();
        if let Some(parent) = favorites_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(json) = serde_json::to_string_pretty(&self.favorites) {
            let _ = std::fs::write(&favorites_path, json);
        }
    }

    pub fn toggle_favorite(&mut self) {
        if !self.in_system || self.roms.is_empty() {
            return;
        }

        let selected_rom = &self.roms[self.selected];
        let favorite = FavoriteEntry {
            system: self.current_system.clone(),
            title: selected_rom.title.clone(),
        };

        // Check if already favorite
        if let Some(pos) = self
            .favorites
            .iter()
            .position(|f| f.system == favorite.system && f.title == favorite.title)
        {
            // Remove from favorites
            self.favorites.remove(pos);
        } else {
            // Add to favorites
            self.favorites.push(favorite);
        }

        self.save_favorites();
    }

    pub fn is_favorite(&self, system: &str, title: &str) -> bool {
        self.favorites
            .iter()
            .any(|f| f.system == system && f.title == title)
    }

    pub fn update_scroll_for_height(&mut self, visible_height: usize) {
        let scroll_threshold_items = 5;

        let near_end = self.roms.len().saturating_sub(scroll_threshold_items);

        let threshold = if self.selected >= near_end {
            0
        } else {
            scroll_threshold_items
        };

        let max_visible_items = visible_height.saturating_sub(threshold);

        self.update_scroll_offset(max_visible_items);
    }
}
