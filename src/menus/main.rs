//! 主菜单（显示在标题屏幕上）。

use bevy::prelude::*;

use crate::{
    FntAssets,
    asset_tracking::ResourceHandles,
    i18n::{
        LanguageRes,
        config::{MAIN_CREDITS, MAIN_EXIT, MAIN_PLAY, MAIN_SETTINGS},
    },
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands, lang_res: Res<LanguageRes>, font_res: Res<FntAssets>) {
    let font = font_res.pixel.clone();
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
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
            widget::button(lang_res.get(MAIN_CREDITS), font.clone(), open_credits_menu),
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
            widget::button(lang_res.get(MAIN_CREDITS), font.clone(), open_credits_menu),
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
