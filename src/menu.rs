use tauri::{Menu, MenuItem, Submenu};

#[warn(dead_code)]
pub fn main_menu_builder() -> Menu {
    Menu::new().add_submenu(Submenu::new(
        "Settings", // TODO Settings
        Menu::new().add_native_item(MenuItem::About(
            "About".to_string(),
            tauri::AboutMetadata::default(),
        )),
    ))
}
