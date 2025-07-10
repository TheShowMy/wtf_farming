use bevy::prelude::*;

use crate::{i18n::config::SELECT_GAME_TITLE, menus::Menu, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::SelectGame), spawn_select_game_menu);
}

fn spawn_select_game_menu(
    mut commands: Commands,
    lang_res: Res<crate::i18n::LanguageRes>,
    font_res: Res<crate::FntAssets>,
) {
    use bevy::ui::Val::*;
    commands.spawn((
        widget::ui_root("Select Game Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::SelectGame),
        children![
            widget::header(lang_res.get(SELECT_GAME_TITLE), font_res.pixel()),
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

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
