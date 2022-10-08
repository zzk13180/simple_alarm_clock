use tauri::{Menu, MenuItem, Submenu};

pub fn main_menu_builder() -> Menu {
    Menu::new().add_submenu(Submenu::new(
        "Close",
        Menu::new()
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::Quit),
    ))
}
