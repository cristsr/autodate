use crate::tray::item_builder::{TrayItemBuilder, TrayMenuItemType};
use crate::tray::menu::TrayMenu;
use crate::tray::tray::{Tray, TrayIcons};
use crate::tray::types::TrayEvent;
use std::sync::{Arc, Mutex};
use tray_icon::menu::MenuId;

pub struct TrayRef {
    pub running: Arc<Mutex<bool>>,
    pub tray: Arc<Mutex<Tray>>,
    pub tray_menu: Arc<Mutex<TrayMenu>>,
}

impl TrayRef {
    pub fn new(
        running: Arc<Mutex<bool>>,
        tray: Arc<Mutex<Tray>>,
        tray_menu: Arc<Mutex<TrayMenu>>,
    ) -> Self {
        Self {
            running,
            tray,
            tray_menu,
        }
    }

    pub fn toggle_running(&self) -> (String, bool) {
        let mut current = self.running.lock().unwrap();
        *current = !*current;

        let title = match *current {
            true => "Running".to_string(),
            false => "Disabled (Click to re-enable)".to_string(),
        };

        (title, *current)
    }

    pub fn update_menu(&mut self) {
        let (title, checked) = self.toggle_running();

        log::debug!("Updating running state: {}", title);

        self.tray_menu.lock().unwrap().update_item(
            TrayItemBuilder::new()
                .with_id(MenuId::new(TrayEvent::Running.as_str()))
                .with_title(&title)
                .with_checked(checked)
                .build(TrayMenuItemType::Check),
        );

        {
            let menu = self.tray_menu.lock().unwrap();
            self.tray.lock().unwrap().set_menu(&*menu);
        }

        // Actualizar ícono
        self.tray.lock().unwrap().set_icon(match checked {
            true => TrayIcons::GREEN,
            false => TrayIcons::RED,
        });
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}
