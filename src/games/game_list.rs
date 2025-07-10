//! 游戏列表

use std::str;

use bevy::prelude::*;

use crate::i18n::config::{GAMES_DESCRIPTION_1, GAMES_NAME_1};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GameList>();
    app.insert_resource(load_game_list());
}

/// 游戏列表资源
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct GameList {
    pub games: Vec<GameItem>,
}

#[derive(Clone, Reflect)]
struct GameItem {
    /// 游戏名称
    pub name: String,
    /// 游戏描述
    pub description: String,
    /// 游戏图标路径
    pub icon: String,
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
