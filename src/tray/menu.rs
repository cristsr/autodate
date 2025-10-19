use crate::tray::constants::{ICON_GREEN, MENU_EXIT, MENU_RUNNING, MENU_TITLE};
use crate::tray::events::TrayEvent;
use crate::tray::item_builder::{TrayItemBuilder, TrayMenuItemType};
use tray_icon::menu::{ContextMenu, IsMenuItem, Menu, MenuId};

pub struct TrayMenu {
    menu: Menu,
}

impl Clone for TrayMenu {
    fn clone(&self) -> Self {
        Self {
            menu: self.menu.clone(),
        }
    }
}

impl Default for TrayMenu {
    fn default() -> Self {
        let mut menu = Self::new();

        menu.add_item(
            TrayItemBuilder::new()
                .with_id(MenuId::new(TrayEvent::Title.as_str()))
                .with_title(MENU_TITLE)
                .with_icon(ICON_GREEN)
                .build(TrayMenuItemType::Icon),
        );

        menu.add_item(
            TrayItemBuilder::new()
                .with_id(MenuId::new(TrayEvent::Running.as_str()))
                .with_title(MENU_RUNNING)
                .with_checked(true)
                .build(TrayMenuItemType::Check),
        );

        menu.add_item(
            TrayItemBuilder::new()
                .with_id(MenuId::new(TrayEvent::Exit.as_str()))
                .with_title(MENU_EXIT)
                .build(TrayMenuItemType::Normal),
        );

        menu
    }
}

impl TrayMenu {
    pub fn new() -> Self {
        Self { menu: Menu::new() }
    }

    pub fn add_item(&mut self, item: Box<dyn IsMenuItem>) {
        self.menu
            .append(item.as_ref())
            .expect("Error adding menu item");
    }

    pub fn update_item(&mut self, item: Box<dyn IsMenuItem>) {
        if let Some(index) = self.menu.items().iter().position(|i| i.id() == item.id()) {
            self.menu.remove_at(index);
            self.menu.insert(item.as_ref(), index).unwrap()
        }
    }

    pub fn as_context_menu(&self) -> Option<Box<dyn ContextMenu>> {
        Some(Box::new(self.menu.clone()))
    }
}
