use crate::tray::constants::{APP_ID, APP_TITLE, APP_TOOLTIP, ICON_GREEN};
use crate::tray::menu::TrayMenu;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

#[derive(Clone)]
pub struct Tray {
    pub tray: Option<TrayIcon>,
}

impl Default for Tray {
    fn default() -> Self {
        let default = TrayIconBuilder::new()
            .with_id(APP_ID)
            .with_icon(Icon::from_resource(ICON_GREEN, None).unwrap())
            .with_tooltip(APP_TOOLTIP)
            .with_title(APP_TITLE)
            .with_menu_on_left_click(true)
            .build()
            .unwrap();

        Tray {
            tray: Some(default),
        }
    }
}

impl Tray {
    pub fn new(menu: &TrayMenu) -> Self {
        let mut tray = Self::default();
        tray.set_menu(menu);
        tray
    }

    pub fn set_menu(&mut self, menu: &TrayMenu) {
        self.tray.as_mut().unwrap().set_menu(menu.as_context_menu());
    }

    pub fn set_icon(&mut self, icon: u16) {
        self.tray
            .as_mut()
            .unwrap()
            .set_icon(Some(Icon::from_resource(icon, None).unwrap()))
            .unwrap();
    }
}
