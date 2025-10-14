use tray_icon::menu::{CheckMenuItem, Icon, IconMenuItem, IsMenuItem, MenuId, MenuItem};

pub enum TrayMenuItemType {
    Check,
    Icon,
    Normal,
}

pub struct TrayItemBuilder {
    id: Option<MenuId>,
    title: Option<String>,
    enabled: bool,
    checked: bool,
    icon: Option<Icon>,
}

impl TrayItemBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            icon: None,
            enabled: true,
            checked: false,
        }
    }

    pub fn with_id(mut self, id: MenuId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_icon(mut self, icon: u16) -> Self {
        self.icon = Some(Icon::from_resource(icon, None).unwrap());
        self
    }

    pub fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn build(self, item_type: TrayMenuItemType) -> Box<dyn IsMenuItem> {
        match item_type {
            TrayMenuItemType::Normal => Box::new(MenuItem::with_id(
                self.id.unwrap(),
                self.title.unwrap(),
                self.enabled,
                None,
            )),
            TrayMenuItemType::Icon => Box::new(IconMenuItem::with_id(
                self.id.unwrap(),
                self.title.unwrap(),
                self.enabled,
                self.icon,
                None,
            )),
            TrayMenuItemType::Check => Box::new(CheckMenuItem::with_id(
                self.id.unwrap(),
                self.title.unwrap(),
                self.enabled,
                self.checked,
                None,
            )),
        }
    }
}
