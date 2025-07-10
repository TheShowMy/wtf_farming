//! 加载屏幕，在此期间如果有必要会加载游戏资源。
//! 这可以减少卡顿，尤其是在 Wasm 上的音频。

use bevy::prelude::*;

use crate::{FntAssets, asset_tracking::ResourceHandles, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_loading_screen);
    app.add_systems(OnEnter(Screen::RefreshLanguage), spawn_loading_screen);
    app.add_systems(OnExit(Screen::Loading), remove_loading_screen);
    app.add_systems(OnExit(Screen::RefreshLanguage), remove_loading_screen);

    app.add_systems(
        Update,
        (
            enter_gameplay_screen.run_if(in_state(Screen::Loading).and(all_assets_loaded)),
            enter_title_screen.run_if(in_state(Screen::RefreshLanguage).and(all_assets_loaded)),
        ),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingScreen;

fn spawn_loading_screen(mut commands: Commands, font_res: Res<FntAssets>) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        LoadingScreen,
        children![widget::label("Loading...", font_res.pixel())],
    ));
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn enter_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}

fn remove_loading_screen(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
