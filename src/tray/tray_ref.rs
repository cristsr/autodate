use crate::tray::constants::{ICON_GREEN, ICON_RED, MENU_DISABLED, MENU_RUNNING, MENU_TITLE};
use crate::tray::events::TrayEvent;
use crate::tray::item_builder::{TrayItemBuilder, TrayMenuItemType};
use crate::tray::menu::TrayMenu;
use crate::tray::tray::Tray;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, atomic};
use tray_icon::menu::MenuId;

pub struct TrayRef {
    pub running: AtomicBool,
    pub tray: Arc<Mutex<Tray>>,
    pub tray_menu: Arc<Mutex<TrayMenu>>,
}

impl TrayRef {
    pub fn new(tray: Arc<Mutex<Tray>>, tray_menu: Arc<Mutex<TrayMenu>>) -> Self {
        Self {
            running: AtomicBool::new(true),
            tray,
            tray_menu,
        }
    }

    pub fn toggle_running(&self) -> bool {
        let next_value = !self.is_running();
        self.set_running(next_value);
        next_value
    }

    pub fn update_menu(&mut self) {
        let is_running = self.toggle_running();

        let (title, icon) = if is_running {
            (MENU_RUNNING, ICON_GREEN)
        } else {
            (MENU_DISABLED, ICON_RED)
        };

        self.tray_menu.lock().unwrap().update_item(
            TrayItemBuilder::new()
                .with_id(MenuId::new(TrayEvent::Title.as_str()))
                .with_title(MENU_TITLE)
                .with_icon(icon)
                .build(TrayMenuItemType::Icon),
        );

        self.tray_menu.lock().unwrap().update_item(
            TrayItemBuilder::new()
                .with_id(MenuId::new(TrayEvent::Running.as_str()))
                .with_title(title)
                .with_checked(is_running)
                .build(TrayMenuItemType::Check),
        );

        let menu = self.tray_menu.lock().unwrap();
        self.tray.lock().unwrap().set_menu(&*menu);

        self.tray.lock().unwrap().set_icon(icon);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(atomic::Ordering::Relaxed)
    }

    pub fn set_running(&self, running: bool) {
        self.running.store(running, atomic::Ordering::Relaxed);
    }
}
