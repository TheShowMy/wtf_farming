//! 主菜单（显示在标题屏幕上）。

use bevy::prelude::*;

use crate::{
    FntAssets,
    asset_tracking::ResourceHandles,
    i18n::{LanguageId, LanguageRes, config::*},
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(
    mut commands: Commands,
    lang_res: Res<LanguageRes>,
    font_res: Res<FntAssets>,
    asset_server: Res<AssetServer>,
) {
    let font = font_res.pixel.clone();
    use bevy::ui::Val::*;
    commands.spawn((
        Name::new("Main Menu"),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        Pickable::IGNORE,
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        children![
            (
                Name::new("Main Left Spacer"),
                Node {
                    width: Percent(3.0),
                    height: Percent(100.0),
                    ..default()
                }
            ),
            (
                Name::new("Main Left Menu"),
                Node {
                    width: Percent(30.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(20.0),
                    ..default()
                },
                children![
                    (
                        Name::new("Top"),
                        Node {
                            width: Percent(100.0),
                            height: Percent(90.0),
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: Px(20.0),
                            ..default()
                        },
                        #[cfg(not(target_family = "wasm"))]
                        children![
                            widget::button(
                                lang_res.get(MAIN_PLAY),
                                font.clone(),
                                enter_loading_or_gameplay_screen,
                            ),
                            widget::button(
                                lang_res.get(MAIN_SETTINGS),
                                font.clone(),
                                open_settings_menu,
                            ),
                            widget::button(
                                lang_res.get(MAIN_CREDITS),
                                font.clone(),
                                open_credits_menu
                            ),
                            widget::button(lang_res.get(MAIN_EXIT), font.clone(), exit_app),
                        ],
                        #[cfg(target_family = "wasm")]
                        children![
                            widget::button(
                                lang_res.get(MAIN_PLAY),
                                font.clone(),
                                enter_loading_or_gameplay_screen
                            ),
                            widget::button(
                                lang_res.get(MAIN_SETTINGS),
                                font.clone(),
                                open_settings_menu
                            ),
                            widget::button(
                                lang_res.get(MAIN_CREDITS),
                                font.clone(),
                                open_credits_menu
                            ),
                        ],
                    ),
                    (
                        Name::new("Bottom"),
                        Node {
                            width: Percent(100.0),
                            height: Percent(7.0),
                            ..default()
                        },
                        children![
                            ImageNode::new(asset_server.load("images/icon/language.png")),
                            widget::button_size(
                                lang_res.curr_language.to_string(),
                                font.clone(),
                                Vec2::new(190.0, 40.0),
                                click_language_button
                            ),
                        ],
                    )
                ],
            ),
            (
                Name::new("Main right Menu"),
                Node {
                    width: VMax(100.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![widget::label_size_80(
                    lang_res.get(GAME_TITLE),
                    font.clone(),
                ),],
            ),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn click_language_button(
    _: Trigger<Pointer<Click>>,
    mut lang_res: ResMut<LanguageRes>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let next_language = if lang_res.curr_language == LanguageId::ZhCn {
        LanguageId::EnUs
    } else {
        LanguageId::ZhCn
    };
    lang_res.set_language(next_language);
    next_screen.set(Screen::RefreshLanguage);
}
