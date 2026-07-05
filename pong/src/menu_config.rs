use bevy::prelude::*;

#[derive(Clone)]
pub enum MenuItem {
    Action { label: String, action: MenuActionMessage },
    Submenu { label: String, items: Vec<MenuItem> },
    Toggle { label: String, enabled: bool },
    Slider { label: String, value: f32, min: f32, max: f32 },
    Select { label: String, options: Vec<String>, selected: usize },
}

#[derive(Message, Clone)]
pub enum MenuActionMessage {
    StartGame,
    Credits,
    Quit,
    Back,
    Custom(String),
}

#[derive(Resource)]
pub struct MenuDefinition {
    pub title: String,
    pub items: Vec<MenuItem>,
}

#[derive(Resource, Default)]
pub struct MenuNavigation {
    pub path: Vec<(usize, usize)>,
    pub selection: usize,
}

#[derive(Component)]
pub struct MenuItemRef(pub usize);
