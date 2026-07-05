use bevy::prelude::*;
use crate::components::{GameState, MenuRoot};
use crate::menu_config::{MenuActionMessage, MenuDefinition, MenuItem, MenuItemRef, MenuNavigation};

pub struct MenuPlugin;

#[derive(Resource, Default)]
struct MenuNeedsRebuild(bool);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuNavigation>()
            .init_resource::<MenuNeedsRebuild>()
            .init_resource::<Messages<MenuActionMessage>>()
            .add_systems(OnEnter(GameState::Menu), (reset_navigation, spawn_menu))
            .add_systems(OnEnter(GameState::Paused), (reset_navigation, spawn_menu))
            .add_systems(
                Update,
                navigate_menu.run_if(
                    in_state(GameState::Menu).or(in_state(GameState::Paused)),
                ),
            )
            .add_systems(
                Update,
                activate_selected.run_if(
                    in_state(GameState::Menu).or(in_state(GameState::Paused)),
                ),
            )
            .add_systems(
                Update,
                handle_menu_interaction.run_if(
                    in_state(GameState::Menu).or(in_state(GameState::Paused)),
                ),
            )
            .add_systems(
                Update,
                menu_respawner.run_if(
                    in_state(GameState::Menu).or(in_state(GameState::Paused)),
                ),
            )
            .add_systems(
                Update,
                update_menu_labels.run_if(
                    in_state(GameState::Menu).or(in_state(GameState::Paused)),
                ),
            )
            .add_systems(
                Update,
                update_menu_button_style.run_if(
                    in_state(GameState::Menu).or(in_state(GameState::Paused)),
                ),
            )
            .add_systems(
                Update,
                handle_menu_action_events.run_if(
                    in_state(GameState::Menu).or(in_state(GameState::Paused)),
                ),
            )
            .add_systems(OnExit(GameState::Menu), despawn_menu)
            .add_systems(OnExit(GameState::Paused), despawn_menu);
    }
}

fn current_items<'a>(definition: &'a MenuDefinition, navigation: &MenuNavigation) -> &'a [MenuItem] {
    let mut items = &definition.items;
    for &(i, _) in &navigation.path {
        if let MenuItem::Submenu { items: sub, .. } = &items[i] {
            items = sub;
        }
    }
    items
}

fn current_title<'a>(definition: &'a MenuDefinition, navigation: &MenuNavigation) -> &'a str {
    if navigation.path.is_empty() {
        return &definition.title;
    }
    let mut items = &definition.items;
    let mut label = &definition.title;
    for &(i, _) in &navigation.path {
        if let MenuItem::Submenu { label: lbl, items: sub, .. } = &items[i] {
            label = lbl;
            items = sub;
        }
    }
    label
}

fn item_label(item: &MenuItem) -> String {
    match item {
        MenuItem::Action { label, .. } => label.clone(),
        MenuItem::Submenu { label, .. } => format!("{} >", label),
        MenuItem::Toggle { label, enabled } => {
            format!("{}: {}", label, if *enabled { "ON" } else { "OFF" })
        }
        MenuItem::Slider { label, value, .. } => format!("{}: {:.0}", label, value),
        MenuItem::Select { label, options, selected } => {
            format!("{}: {}", label, options[*selected])
        }
    }
}

fn spawn_menu_inner(commands: &mut Commands, definition: &MenuDefinition, navigation: &MenuNavigation) {
    let items = current_items(definition, navigation);
    let title = current_title(definition, navigation);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        MenuRoot,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(title.to_string()),
            TextFont { font_size: 80.0, ..default() },
            TextColor(Color::WHITE),
            TextLayout::new(Justify::Center, LineBreak::NoWrap),
            Node {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ));

        for (index, item) in items.iter().enumerate() {
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                MenuItemRef(index),
            )).with_children(|button_parent| {
                button_parent.spawn((
                    Text::new(item_label(item)),
                    TextFont { font_size: 36.0, ..default() },
                    TextColor(Color::WHITE),
                    TextLayout::new(Justify::Center, LineBreak::NoWrap),
                ));
            });
        }
    });
}

fn reset_navigation(mut navigation: ResMut<MenuNavigation>) {
    navigation.path.clear();
    navigation.selection = 0;
}

fn spawn_menu(mut commands: Commands, definition: Res<MenuDefinition>, navigation: Res<MenuNavigation>) {
    spawn_menu_inner(&mut commands, &definition, &navigation);
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn navigate_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut navigation: ResMut<MenuNavigation>,
    mut definition: ResMut<MenuDefinition>,
    buttons: Query<(), With<MenuItemRef>>,
) {
    let count = buttons.iter().len();
    if count == 0 {
        return;
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        navigation.selection = (navigation.selection + 1) % count;
        return;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        navigation.selection = if navigation.selection == 0 {
            count - 1
        } else {
            navigation.selection - 1
        };
        return;
    }

    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::ArrowRight) {
        let delta = if keys.just_pressed(KeyCode::ArrowRight) { 1 } else { -1 };
        let path = navigation.path.clone();
        let sel = navigation.selection;
        modify_value_at_path(&mut definition.items, &path, sel, delta);
    }
}

fn modify_value_at_path(
    items: &mut Vec<MenuItem>,
    path: &[(usize, usize)],
    selection: usize,
    delta: isize,
) {
    let mut current = items;
    for &(i, _) in path {
        if let Some(MenuItem::Submenu { items: sub, .. }) = current.get_mut(i) {
            current = sub;
        } else {
            return;
        }
    }
    if selection < current.len() {
        apply_delta(&mut current[selection], delta);
    }
}

fn apply_delta(item: &mut MenuItem, delta: isize) {
    match item {
        MenuItem::Toggle { enabled, .. } => *enabled = !*enabled,
        MenuItem::Slider { value, min, max, .. } => {
            let step = (*max - *min) / 20.0;
            *value = (*value + delta as f32 * step).clamp(*min, *max);
        }
        MenuItem::Select { selected, options, .. } => {
            *selected = ((*selected as isize + delta).rem_euclid(options.len() as isize)) as usize;
        }
        _ => {}
    }
}

fn activate_selected(
    keys: Res<ButtonInput<KeyCode>>,
    mut navigation: ResMut<MenuNavigation>,
    mut definition: ResMut<MenuDefinition>,
    mut events: MessageWriter<MenuActionMessage>,
    mut needs_rebuild: ResMut<MenuNeedsRebuild>,
) {
    if !keys.just_pressed(KeyCode::Enter) && !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let path = navigation.path.clone();
    let sel = navigation.selection;
    activate_at_path(&mut definition.items, &path, sel, &mut navigation, &mut events, &mut needs_rebuild);
}

fn activate_at_path(
    items: &mut Vec<MenuItem>,
    path: &[(usize, usize)],
    selection: usize,
    navigation: &mut MenuNavigation,
    events: &mut MessageWriter<MenuActionMessage>,
    needs_rebuild: &mut MenuNeedsRebuild,
) {
    let mut current = items;
    for &(i, _) in path {
        if let Some(MenuItem::Submenu { items: sub, .. }) = current.get_mut(i) {
            current = sub;
        } else {
            return;
        }
    }
    if selection >= current.len() {
        return;
    }

    match &mut current[selection] {
        MenuItem::Submenu { .. } => {
            navigation.path.push((selection, navigation.selection));
            navigation.selection = 0;
            needs_rebuild.0 = true;
        }
        MenuItem::Action { action, .. } => {
            if matches!(action, MenuActionMessage::Back) {
                if let Some((_, saved_selection)) = navigation.path.pop() {
                    navigation.selection = saved_selection;
                    needs_rebuild.0 = true;
                }
            } else {
                events.write(action.clone());
            }
        }
        _ => {
            apply_delta(&mut current[selection], 1);
        }
    }
}

fn handle_menu_interaction(
    mut navigation: ResMut<MenuNavigation>,
    mut definition: ResMut<MenuDefinition>,
    mut events: MessageWriter<MenuActionMessage>,
    mut needs_rebuild: ResMut<MenuNeedsRebuild>,
    query: Query<(&Interaction, &MenuItemRef), (With<Button>, Changed<Interaction>)>,
) {
    for (interaction, item_ref) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let path = navigation.path.clone();
        activate_at_path(
            &mut definition.items,
            &path,
            item_ref.0,
            &mut navigation,
            &mut events,
            &mut needs_rebuild,
        );
    }
}

fn menu_respawner(
    mut commands: Commands,
    definition: Res<MenuDefinition>,
    navigation: Res<MenuNavigation>,
    mut needs_rebuild: ResMut<MenuNeedsRebuild>,
    root_query: Query<Entity, With<MenuRoot>>,
) {
    if !needs_rebuild.0 {
        return;
    }
    needs_rebuild.0 = false;

    for entity in root_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_menu_inner(&mut commands, &definition, &navigation);
}

fn update_menu_labels(
    definition: Res<MenuDefinition>,
    navigation: Res<MenuNavigation>,
    buttons: Query<(&MenuItemRef, &Children)>,
    mut texts: Query<&mut Text>,
) {
    let items = current_items(&definition, &navigation);
    for (item_ref, children) in buttons.iter() {
        if item_ref.0 >= items.len() {
            continue;
        }
        let label = item_label(&items[item_ref.0]);
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                *text = Text::new(label.clone());
            }
        }
    }
}

fn update_menu_button_style(
    navigation: Res<MenuNavigation>,
    mut buttons: Query<(&Interaction, &MenuItemRef, &mut BackgroundColor, &Children), With<Button>>,
    mut texts: Query<&mut TextColor>,
) {
    for (interaction, item_ref, mut bg_color, children) in buttons.iter_mut() {
        let is_selected = item_ref.0 == navigation.selection;

        let new_bg = if *interaction == Interaction::Pressed {
            Color::srgba(1.0, 1.0, 1.0, 0.3)
        } else if *interaction == Interaction::Hovered {
            Color::srgba(1.0, 1.0, 1.0, 0.15)
        } else if is_selected {
            Color::srgba(1.0, 1.0, 1.0, 0.1)
        } else {
            Color::NONE
        };

        let new_text_color = if is_selected && *interaction == Interaction::None {
            Color::srgb(1.0, 0.9, 0.5)
        } else {
            Color::WHITE
        };

        bg_color.0 = new_bg;

        for child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                text_color.0 = new_text_color;
            }
        }
    }
}

fn handle_menu_action_events(
    mut events: MessageReader<MenuActionMessage>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for event in events.read() {
        match event {
            MenuActionMessage::StartGame => next_state.set(GameState::InGame),
            MenuActionMessage::Credits => next_state.set(GameState::Credits),
            MenuActionMessage::Quit => { exit.write(AppExit::Success); },
            _ => {}
        }
    }
}
