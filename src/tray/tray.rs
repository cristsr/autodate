use crate::tray::menu::TrayMenu;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

#[derive(Debug)]
#[repr(u16)]
pub enum TrayIcons {
    MAIN = 1,
    GREEN = 2,
    RED = 3,
}

pub struct TrayLabels {
    id: &'static str,
    title: &'static str,
    tooltip: &'static str,
}

const LABELS: TrayLabels = TrayLabels {
    id: "Bill Renamer",
    title: "Bill Renamer",
    tooltip: "Bill Renamer",
};

#[derive(Clone)]
pub struct Tray {
    pub tray: Option<TrayIcon>,
}

impl Default for Tray {
    fn default() -> Self {
        let default = TrayIconBuilder::new()
            .with_id(LABELS.id)
            .with_icon(Icon::from_resource(TrayIcons::MAIN as u16, None).unwrap())
            .with_tooltip(LABELS.tooltip)
            .with_title(LABELS.title)
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

    pub fn set_icon(&mut self, icon: TrayIcons) {
        self.tray
            .as_mut()
            .unwrap()
            .set_icon(Some(Icon::from_resource(icon as u16, None).unwrap()))
            .unwrap();
    }
}
