use bevy::{
    ecs::spawn::SpawnWith,
    picking::{
        hover::HoverMap,
        pointer::{PointerAction, PointerInput},
    },
    prelude::*,
};

use crate::{
    AppSystems,
    games::game_list::{GameList, OpenGameEvent},
    i18n::config::SELECT_GAME_TITLE,
    menus::Menu,
    theme::{
        palette::{BUTTON_PRESSED_BACKGROUND, HEADER_TEXT},
        widget,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::SelectGame), spawn_select_game_menu);
    app.add_systems(
        Update,
        update_scroll_position
            .run_if(in_state(Menu::SelectGame))
            .in_set(AppSystems::RecordInput),
    );
}

fn spawn_select_game_menu(
    mut commands: Commands,
    lang_res: Res<crate::i18n::LanguageRes>,
    font_res: Res<crate::FntAssets>,
    game_list: Res<GameList>,
) {
    use bevy::ui::Val::*;
    let game_items = game_list
        .games
        .iter()
        .enumerate()
        .map(|(index, item)| {
            (
                lang_res.get(&item.name),
                lang_res.get(&item.description), // Assuming name is unique
                index,
            )
        })
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .get_or_insert_default()
        .clone();

    commands.spawn((
        widget::ui_root("Select Game Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::SelectGame),
        children![
            widget::header(lang_res.get(SELECT_GAME_TITLE), font_res.pixel()),
            (
                Name::new("Game Button List"),
                Node {
                    width: Percent(95.0),
                    height: Auto,
                    flex_direction: FlexDirection::Row,
                    overflow: Overflow::scroll_x(), // n.b.
                    column_gap: Px(10.0),
                    ..default()
                },
                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                    for index in 0..20 {
                        parent
                            .spawn((
                                Name::new(format!("Game Button {index}")),
                                Button,
                                Node {
                                    min_width: Px(200.0),
                                    height: Px(200.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                BackgroundColor(HEADER_TEXT),
                                Pickable {
                                    should_block_lower: false,
                                    ..default()
                                },
                                children![(
                                    Text::new(game_items.0.to_string()),
                                    TextFont::from_font_size(24.0),
                                    TextColor::from(BUTTON_PRESSED_BACKGROUND),
                                    Pickable::IGNORE,
                                ),],
                            ))
                            .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                                commands.trigger(OpenGameEvent {
                                    index: game_items.2 as u8,
                                });
                            });
                    }
                })),
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: Px(400.0),
                    height: Px(200.0),
                    left: Percent(1.0),
                    top: Percent(1.0),
                    ..default()
                },
                children![widget::button(
                    lang_res.get(crate::i18n::config::PAUSE_QUIT_TO_TITLE),
                    font_res.pixel(),
                    go_back_on_click,
                )],
            )
        ],
    ));
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

pub fn update_scroll_position(
    mut mouse_pointer_events: MessageReader<PointerInput>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
) {
    for event in mouse_pointer_events.read() {
        let (_, y) = match event.action {
            PointerAction::Scroll { unit: _, x, y } => (x, y),
            _ => continue,
        };
        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.x -= -y;
                    scroll_position.y -= 0.;
                }
            }
        }
    }
}
