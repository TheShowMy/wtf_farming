//! 暂停菜单。

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    FntAssets,
    i18n::{
        LanguageRes,
        config::{MAIN_SETTINGS, PAUSE_CONTINUE, PAUSE_GAME_TITLE, PAUSE_QUIT_TO_TITLE},
    },
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands, font_res: Res<FntAssets>, lang_res: Res<LanguageRes>) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Pause),
        children![
            widget::header(lang_res.get(PAUSE_GAME_TITLE), font_res.pixel()),
            widget::button(lang_res.get(PAUSE_CONTINUE), font_res.pixel(), close_menu),
            widget::button(
                lang_res.get(MAIN_SETTINGS),
                font_res.pixel(),
                open_settings_menu
            ),
            widget::button(
                lang_res.get(PAUSE_QUIT_TO_TITLE),
                font_res.pixel(),
                quit_to_title
            ),
        ],
    ));
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
