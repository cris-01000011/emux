use tui_input::Input;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputActive {
    #[default]
    None,
    Search,
    NewListName,
    NewListUrl,
}

#[derive(Default)]
pub struct Inputs {
    pub active: InputActive,
    pub search: Input,
    pub new_list_name: Input,
    pub new_list_url: Input,
}
