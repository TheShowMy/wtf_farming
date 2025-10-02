//! 游戏列表

use std::str;

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    i18n::config::{GAMES_DESCRIPTION_1, GAMES_NAME_1},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<OpenGameEvent>();
    app.register_type::<GameList>();
    app.insert_resource(load_game_list());
    app.add_observer(open_game_event_handler);
}

/// 游戏列表资源
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameList {
    pub games: Vec<GameItem>,
}

#[derive(Clone, Reflect)]
pub struct GameItem {
    /// 游戏名称
    pub name: String,
    /// 游戏描述
    pub description: String,
    /// 游戏图标路径
    pub icon: String,
}

///打开游戏事件
#[derive(Event, Message, Clone)]
pub struct OpenGameEvent {
    pub index: u8,
}

fn load_game_list() -> GameList {
    // 这里可以从配置文件或其他资源加载游戏列表
    // 例如，假设我们有一个 JSON 文件包含游戏列表
    let games = vec![GameItem {
        name: GAMES_NAME_1.to_string(),
        description: GAMES_DESCRIPTION_1.to_string(),
        icon: "assets/icons/game1.png".to_string(),
    }];
    GameList { games }
}

fn open_game_event_handler(
    open_game_triggers: On<OpenGameEvent>,
    game_list: Res<GameList>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let event = open_game_triggers.event();
    if let Some(game) = game_list.games.get(event.index as usize) {
        info!("Opening game: {}", game.name);
        if resource_handles.is_all_done() {
            next_screen.set(Screen::Gameplay);
        } else {
            next_screen.set(Screen::Loading);
        }
    }
}
