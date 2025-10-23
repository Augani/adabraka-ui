//! Native application menu bar builder for desktop applications.

use gpui::{Menu, MenuItem, SharedString, SystemMenuType, Styled, StyleRefinement};

pub struct AppMenuBar {
    menus: Vec<Menu>,
}

impl AppMenuBar {
    pub fn new() -> Self {
        Self { menus: Vec::new() }
    }

    pub fn menu(mut self, menu: AppMenu) -> Self {
        self.menus.push(menu.build());
        self
    }

    pub fn build(self) -> Vec<Menu> {
        self.menus
    }
}

impl Default for AppMenuBar {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AppMenu {
    name: SharedString,
    items: Vec<MenuItem>,
    style: StyleRefinement,
}

impl AppMenu {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            items: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn action<A: gpui::Action>(mut self, label: impl Into<SharedString>, action: A) -> Self {
        self.items.push(MenuItem::action(label.into(), action));
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::separator());
        self
    }

    pub fn submenu(mut self, submenu: AppMenu) -> Self {
        let submenu_built = submenu.build();
        self.items.push(MenuItem::submenu(submenu_built));
        self
    }

    pub fn os_submenu(mut self, label: impl Into<SharedString>, menu_type: SystemMenuType) -> Self {
        self.items.push(MenuItem::os_submenu(label.into(), menu_type));
        self
    }

    pub fn build(self) -> Menu {
        Menu {
            name: self.name,
            items: self.items,
        }
    }
}

impl Styled for AppMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

pub fn file_menu() -> AppMenu {
    AppMenu::new("File")
}

pub fn edit_menu() -> AppMenu {
    AppMenu::new("Edit")
}

pub fn view_menu() -> AppMenu {
    AppMenu::new("View")
}

pub fn window_menu() -> AppMenu {
    AppMenu::new("Window")
}

pub fn help_menu() -> AppMenu {
    AppMenu::new("Help")
}

pub struct StandardMacMenuBar {
    app_name: SharedString,
    file_menu: Option<AppMenu>,
    edit_menu: Option<AppMenu>,
    view_menu: Option<AppMenu>,
    window_menu: Option<AppMenu>,
    help_menu: Option<AppMenu>,
}

impl StandardMacMenuBar {
    pub fn new(app_name: impl Into<SharedString>) -> Self {
        Self {
            app_name: app_name.into(),
            file_menu: None,
            edit_menu: None,
            view_menu: None,
            window_menu: None,
            help_menu: None,
        }
    }

    pub fn file_menu(mut self, menu: AppMenu) -> Self {
        self.file_menu = Some(menu);
        self
    }

    pub fn edit_menu(mut self, menu: AppMenu) -> Self {
        self.edit_menu = Some(menu);
        self
    }

    pub fn view_menu(mut self, menu: AppMenu) -> Self {
        self.view_menu = Some(menu);
        self
    }

    pub fn window_menu(mut self, menu: AppMenu) -> Self {
        self.window_menu = Some(menu);
        self
    }

    pub fn help_menu(mut self, menu: AppMenu) -> Self {
        self.help_menu = Some(menu);
        self
    }

    pub fn build(self) -> Vec<Menu> {
        let mut menus = Vec::new();

        #[cfg(target_os = "macos")]
        {
            let app_menu = AppMenu::new(&self.app_name)
                .os_submenu("Services", SystemMenuType::Services)
                .separator();
            menus.push(app_menu.build());
        }

        if let Some(file_menu) = self.file_menu {
            menus.push(file_menu.build());
        }

        if let Some(edit_menu) = self.edit_menu {
            menus.push(edit_menu.build());
        }

        if let Some(view_menu) = self.view_menu {
            menus.push(view_menu.build());
        }

        if let Some(window_menu) = self.window_menu {
            menus.push(window_menu.build());
        }

        if let Some(help_menu) = self.help_menu {
            menus.push(help_menu.build());
        }

        menus
    }
}
